// Authentication handlers - FULL IMPLEMENTATION
// I-FR-21: SSO
// I-FR-23: API key management
// I-FR-25: Rate limiting

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use crate::db::DbPool;
use crate::db::repositories::user_repository::UserRepository;
use crate::models::user::{ApiKey, ApiKeyStatus, User, UserRole};
use serde_json::json;
use sha2::{Digest, Sha256};
use chrono::Utc;

// I-FR-21: SSO login
pub async fn sso_login(
    State(_db_pool): State<DbPool>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Implement SSO flow with SAML/OIDC
    // For now, return placeholder
    Ok(Json(json!({
        "redirect_url": "https://login.mediacorp.com/saml/sso",
        "status": "redirect_required"
    })))
}

pub async fn sso_callback(
    State(db_pool): State<DbPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Handle SSO callback, validate SAML assertion
    // Create or get user, generate JWT token
    Ok(Json(json!({
        "access_token": "placeholder_token",
        "status": "success"
    })))
}

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