// JWT utility functions
// Generate and validate JWT tokens

use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, Algorithm};
use crate::middleware::auth::Claims;
use anyhow::Result;

pub struct JWTService;

impl JWTService {
    /// Get JWT secret from environment
    pub fn get_secret() -> String {
        std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| {
                tracing::warn!("JWT_SECRET not set, using default (INSECURE - change in production!)");
                "change-me-in-production-default-secret-key".to_string()
            })
    }
    
    /// Generate JWT access token
    pub fn generate_access_token(user_id: &str, email: &str, role: &str) -> Result<String> {
        let secret = Self::get_secret();
        let expiration = chrono::Utc::now() + chrono::Duration::hours(24);
        
        let claims = Claims {
            user_id: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            exp: expiration.timestamp() as usize,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;
        
        Ok(token)
    }
    
    /// Generate JWT refresh token (longer expiry)
    pub fn generate_refresh_token(user_id: &str, email: &str, role: &str) -> Result<String> {
        let secret = Self::get_secret();
        let expiration = chrono::Utc::now() + chrono::Duration::days(30);
        
        let claims = Claims {
            user_id: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            exp: expiration.timestamp() as usize,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;
        
        Ok(token)
    }
    
    /// Validate JWT token
    pub fn validate_token(token: &str) -> Result<Claims> {
        let secret = Self::get_secret();
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        
        let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
        
        // Check expiration manually
        let now = chrono::Utc::now().timestamp() as usize;
        if token_data.claims.exp < now {
            return Err(anyhow::anyhow!("Token expired"));
        }
        
        Ok(token_data.claims)
    }
    
    /// Generate a secure random JWT secret (for setup)
    pub fn generate_secret() -> String {
        use sha2::{Digest, Sha256};
        use uuid::Uuid;
        
        // Generate random secret
        let random_data = format!("{}-{}", Uuid::new_v4(), chrono::Utc::now().timestamp());
        let mut hasher = Sha256::new();
        hasher.update(random_data.as_bytes());
        format!("jwt_secret_{:x}", hasher.finalize())
    }
}

