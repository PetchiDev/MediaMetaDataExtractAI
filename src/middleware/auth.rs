// Authentication middleware
// I-FR-21: SSO token validation
// I-FR-23: API key validation

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::db::DbPool;
use crate::db::repositories::user_repository::UserRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub email: String,
    pub role: String,
    pub exp: usize,
}

pub async fn authenticate(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Check for Authorization header
    let auth_header = headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            tracing::warn!("Missing Authorization header");
            StatusCode::UNAUTHORIZED
        })?;

    // Get database pool from extensions (set by state)
    let db_pool = request.extensions()
        .get::<Arc<DbPool>>()
        .ok_or_else(|| {
            tracing::error!("Database pool not found in request extensions");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Check if it's a Bearer token (SSO/JWT) or API key
    if auth_header.starts_with("Bearer ") {
        let token = auth_header.strip_prefix("Bearer ").unwrap();
        
        // Validate JWT token using JWT service
        use crate::utils::jwt::JWTService;
        
        match JWTService::validate_token(token) {
            Ok(claims) => {
                // Add user info to request extensions
                request.extensions_mut().insert(claims);
                Ok(next.run(request).await)
            }
            Err(e) => {
                tracing::warn!("JWT validation failed: {:?}", e);
                Err(StatusCode::UNAUTHORIZED)
            }
        }
    } else if auth_header.starts_with("ApiKey ") {
        // I-FR-23: API key validation
        let api_key = auth_header.strip_prefix("ApiKey ").unwrap();
        
        // Hash the provided key and check against database
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        let key_hash = format!("{:x}", hasher.finalize());
        
        match UserRepository::get_api_key_by_hash(db_pool, &key_hash).await {
            Ok(Some(api_key_record)) => {
                // Update last_used timestamp - use double deref to get &Pool from Arc<Pool>
                let pool: &sqlx::PgPool = &**db_pool;
                sqlx::query("UPDATE api_keys SET last_used = NOW() WHERE id = $1")
                    .bind(&api_key_record.id)
                    .execute(pool)
                    .await
                    .ok(); // Don't fail if update fails
                
                // Get user and add to request extensions
                match UserRepository::get_by_id(&db_pool, api_key_record.user_id).await {
                    Ok(Some(user)) => {
                        let claims = Claims {
                            user_id: user.id.to_string(),
                            email: user.email.clone(),
                            role: format!("{:?}", user.role),
                            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
                        };
                        request.extensions_mut().insert(claims);
                        Ok(next.run(request).await)
                    }
                    Ok(None) => {
                        tracing::warn!("User not found for API key");
                        Err(StatusCode::UNAUTHORIZED)
                    }
                    Err(e) => {
                        tracing::error!("Database error: {:?}", e);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            }
            Ok(None) => {
                tracing::warn!("Invalid API key");
                Err(StatusCode::UNAUTHORIZED)
            }
            Err(e) => {
                tracing::error!("Database error: {:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        tracing::warn!("Invalid Authorization header format");
        Err(StatusCode::UNAUTHORIZED)
    }
}
