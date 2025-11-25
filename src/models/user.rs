// User and authentication models
// I-FR-21: SSO enablement
// I-FR-23: Secure API and access governance
// I-FR-25: Rate limiting

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub sso_provider_id: Option<String>, // I-FR-21: SSO integration
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "UPPERCASE")]
pub enum UserRole {
    Admin,
    ContentManager,
    Editor,
    Developer,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub key_name: String,
    pub key_hash: String, // Stored as hash, never plaintext
    pub permissions: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub status: ApiKeyStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "api_key_status", rename_all = "UPPERCASE")]
pub enum ApiKeyStatus {
    Active,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
    pub expires_in: i64,
}
