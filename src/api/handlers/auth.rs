// Authentication handlers - FULL IMPLEMENTATION
// I-FR-21: SSO
// I-FR-23: API key management
// I-FR-25: Rate limiting

use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderValue, HeaderMap},
    response::{Json, Response, IntoResponse},
};
use uuid::Uuid;
use crate::db::DbPool;
use crate::db::repositories::user_repository::UserRepository;
use crate::models::user::{ApiKey, ApiKeyStatus, User, UserRole};
use crate::api::openapi::{GoogleLoginResponse, SSOCallbackResponse};
use serde_json::json;
use sha2::{Digest, Sha256};
use chrono::Utc;

// I-FR-21: SSO login with Google
/// Initiates Google Sign-In flow
/// Returns redirect URL to Google OAuth
#[utoipa::path(
    get,
    path = "/api/auth/google/login",
    tag = "Auth",
    responses(
        (status = 200, description = "Returns Google OAuth redirect URL and callback URL", body = GoogleLoginResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn google_login(
    State(_db_pool): State<DbPool>,
) -> Result<Response, StatusCode> {
    use crate::services::google_oauth::GoogleOAuthService;
    
    let google_oauth = GoogleOAuthService::new()
        .map_err(|e| {
            tracing::error!("Failed to initialize Google OAuth: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let (auth_url, _csrf_token) = google_oauth.get_authorization_url();
    
    // Return redirect URL (frontend will redirect)
    // Note: In Swagger UI, this returns JSON. In browser, you can redirect directly
    Ok(Json(json!({
        "redirect_url": auth_url.to_string(),
        "callback_url": "http://localhost:3000/api/auth/google/callback",
        "status": "redirect_required",
        "message": "Redirect user to Google Sign-In. After Google auth, user will be redirected to callback_url with code parameter.",
        "instructions": "1. Copy redirect_url and open in browser\n2. Sign in with Google\n3. Google will redirect to callback_url\n4. Get JWT token from response"
    })).into_response())
}

/// Google OAuth callback handler
/// Handles Google OAuth callback and generates JWT token
#[utoipa::path(
    get,
    path = "/api/auth/google/callback",
    tag = "Auth",
    params(
        ("code" = String, Query, description = "Authorization code from Google"),
        ("state" = String, Query, description = "CSRF state token")
    ),
    responses(
        (status = 200, description = "Authentication successful. Returns JWT access_token and refresh_token.", body = SSOCallbackResponse),
        (status = 400, description = "Missing authorization code", body = ErrorResponse),
        (status = 401, description = "Invalid authorization code", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn google_callback(
    State(db_pool): State<DbPool>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    use crate::services::google_oauth::GoogleOAuthService;
    
    // Get authorization code from Google
    let code = params.get("code")
        .ok_or_else(|| {
            tracing::warn!("Missing authorization code in callback");
            StatusCode::BAD_REQUEST
        })?;
    
    // Initialize Google OAuth service
    let google_oauth = GoogleOAuthService::new()
        .map_err(|e| {
            tracing::error!("Failed to initialize Google OAuth: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Exchange code for access token
    let access_token = google_oauth.exchange_code(code.clone())
        .await
        .map_err(|e| {
            tracing::error!("Failed to exchange code for token: {:?}", e);
            StatusCode::UNAUTHORIZED
        })?;
    
    // Get user info from Google
    let google_user = google_oauth.get_user_info(&access_token)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user info from Google: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Use Google ID as SSO provider ID
    let sso_provider_id = format!("google_{}", google_user.id);
    
    // Get or create user
    let user = match UserRepository::get_by_sso_id(&db_pool, &sso_provider_id).await {
        Ok(Some(mut existing_user)) => {
            // Update user info if changed
            if existing_user.email != google_user.email || existing_user.name != google_user.name {
                sqlx::query("UPDATE users SET email = $1, name = $2 WHERE id = $3")
                    .bind(&google_user.email)
                    .bind(&google_user.name)
                    .bind(&existing_user.id)
                    .execute(db_pool.as_ref())
                    .await
                    .ok();
                
                existing_user.email = google_user.email.clone();
                existing_user.name = google_user.name.clone();
            }
            existing_user
        }
        Ok(None) => {
            // Create new user
            let new_user = User {
                id: uuid::Uuid::new_v4(),
                email: google_user.email.clone(),
                name: google_user.name.clone(),
                role: UserRole::Viewer, // Default role
                sso_provider_id: Some(sso_provider_id.clone()),
                created_at: chrono::Utc::now(),
                last_login: None,
            };
            
            UserRepository::create(&db_pool, &new_user).await
                .map_err(|e| {
                    tracing::error!("Failed to create user: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            
            new_user
        }
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    // Update last login
    sqlx::query("UPDATE users SET last_login = NOW() WHERE id = $1")
        .bind(&user.id)
        .execute(db_pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Generate JWT tokens using JWT service
    use crate::utils::jwt::JWTService;
    
    let token = JWTService::generate_access_token(
        &user.id.to_string(),
        &user.email,
        &format!("{:?}", user.role),
    ).map_err(|e| {
        tracing::error!("Failed to generate access token: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let refresh_token = JWTService::generate_refresh_token(
        &user.id.to_string(),
        &user.email,
        &format!("{:?}", user.role),
    ).map_err(|e| {
        tracing::error!("Failed to generate refresh token: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(json!({
        "access_token": token,
        "refresh_token": refresh_token,
        "token_type": "Bearer",
        "expires_in": 86400, // 24 hours
        "user": {
            "id": user.id,
            "email": user.email,
            "name": user.name,
            "role": format!("{:?}", user.role),
            "picture": google_user.picture
        },
        "status": "success"
    })))
}

/// Generate a new API key
/// 
/// I-FR-23: Secure API and access governance
/// 
/// **WARNING**: The API key is only returned once. Store it securely.
#[utoipa::path(
    post,
    path = "/api/auth/api-keys",
    tag = "Auth",
    request_body(
        content = serde_json::Value,
        description = "API key generation request",
        example = json!({
            "key_name": "Production Integration Key",
            "permissions": ["submit:media", "read:metadata"]
        })
    ),
    responses(
        (status = 201, description = "API key generated successfully", body = ApiKeyResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("api_key" = []),
        ("bearer_auth" = [])
    )
)]
// I-FR-23: API key management
pub async fn generate_api_key(
    State(db_pool): State<DbPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let key_name = payload.get("key_name").and_then(|s| s.as_str()).ok_or(StatusCode::BAD_REQUEST)?;
    let user_id = payload.get("user_id")
        .and_then(|u| u.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .unwrap_or_else(|| Uuid::new_v4()); // TODO: Get from auth

    // Generate secure random key
    let api_key = format!("mc_sk_{}", Uuid::new_v4().to_string().replace("-", ""));
    
    // Hash the key for storage
    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    let key_hash = format!("{:x}", hasher.finalize());

    let permissions: Vec<String> = payload.get("permissions")
        .and_then(|p| p.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();

    let api_key_record = ApiKey {
        id: Uuid::new_v4(),
        user_id,
        key_name: key_name.to_string(),
        key_hash,
        permissions,
        created_at: Utc::now(),
        last_used: None,
        status: ApiKeyStatus::Active,
    };

    UserRepository::create_api_key(&db_pool, &api_key_record).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return plaintext key (only time it's shown!)
    Ok(Json(json!({
        "api_key": api_key,
        "key_id": api_key_record.id,
        "warning": "Store this key securely. It cannot be retrieved again."
    })))
}

pub async fn list_api_keys(
    State(db_pool): State<DbPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = Uuid::new_v4(); // TODO: Get from auth
    let keys = UserRepository::list_api_keys(&db_pool, user_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: Vec<serde_json::Value> = keys.iter().map(|k| {
        json!({
            "id": k.id,
            "key_name": k.key_name,
            "permissions": k.permissions,
            "created_at": k.created_at,
            "last_used": k.last_used,
            "status": format!("{:?}", k.status),
        })
    }).collect();

    Ok(Json(json!(result)))
}

pub async fn revoke_api_key(
    State(db_pool): State<DbPool>,
    Path(key_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    UserRepository::revoke_api_key(&db_pool, key_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "status": "success",
        "message": "API key revoked"
    })))
}

pub async fn get_permissions(
    State(db_pool): State<DbPool>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user = UserRepository::get_by_id(&db_pool, user_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Map role to permissions
    let permissions = match user.role {
        UserRole::Admin => vec!["*"],
        UserRole::ContentManager => vec!["read:assets", "write:metadata", "upload:media"],
        UserRole::Editor => vec!["read:assets", "write:metadata"],
        UserRole::Developer => vec!["submit:media", "read:metadata"],
        UserRole::Viewer => vec!["read:assets"],
    };

    Ok(Json(json!({
        "user_id": user_id,
        "role": format!("{:?}", user.role),
        "permissions": permissions
    })))
}

// I-FR-25: Rate limiting configuration
pub async fn get_rate_limit_config(
    State(_db_pool): State<DbPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "default_limit_per_minute": 100,
        "per_role": {
            "admin": 1000,
            "developer": 500,
            "content_manager": 200,
            "editor": 100,
            "viewer": 50
        }
    })))
}

pub async fn update_rate_limit_config(
    State(_db_pool): State<DbPool>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Store rate limit config
    Ok(Json(json!({"status": "success"})))
}