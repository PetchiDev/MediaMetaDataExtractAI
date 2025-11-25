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
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
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
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if it's a Bearer token (SSO) or API key
    if auth_header.starts_with("Bearer ") {
        let token = auth_header.strip_prefix("Bearer ").unwrap();
        
        // Validate JWT token (I-FR-21: SSO)
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "change-me-in-production".to_string());
        
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation::new(Algorithm::HS256);
        
        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(token_data) => {
                // Add user info to request extensions
                request.extensions_mut().insert(token_data.claims);
                Ok(next.run(request).await)
            }
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    } else if auth_header.starts_with("ApiKey ") {
        // I-FR-23: API key validation
        let api_key = auth_header.strip_prefix("ApiKey ").unwrap();
        
        // Get database pool from extensions (set by state)
        let db_pool = request.extensions()
            .get::<Arc<DbPool>>()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // Hash the provided key and check against database
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        let key_hash = format!("{:x}", hasher.finalize());
        
        match UserRepository::get_api_key_by_hash(db_pool, &key_hash).await {
            Ok(Some(api_key_record)) => {
                // Update last_used timestamp
                // TODO: Update last_used in database
                
                // Get user and add to request extensions
                if let Ok(Some(user)) = UserRepository::get_by_id(db_pool, api_key_record.user_id).await {
                    let claims = Claims {
                        user_id: user.id.to_string(),
                        email: user.email.clone(),
                        role: format!("{:?}", user.role),
                        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
                    };
                    request.extensions_mut().insert(claims);
                    Ok(next.run(request).await)
                } else {
                    Err(StatusCode::UNAUTHORIZED)
                }
            }
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
