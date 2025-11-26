// Authentication routes
// I-FR-21: SSO enablement
// I-FR-23: Secure API and access governance
// I-FR-25: Rate limiting configuration

use axum::{
    routing::{get, post, delete, put},
    Router,
};
use crate::db::DbPool;

pub fn create_auth_routes(db_pool: DbPool) -> Router {
    Router::new()
        // I-FR-21: Google Sign-In
        .route("/api/auth/google/login", get(crate::api::handlers::auth::google_login))
        .route("/api/auth/google/callback", get(crate::api::handlers::auth::google_callback))
        // I-FR-23: API key management
        .route("/api/access/keys", post(crate::api::handlers::auth::generate_api_key))
        .route("/api/access/keys", get(crate::api::handlers::auth::list_api_keys))
        .route("/api/access/keys/:key_id", delete(crate::api::handlers::auth::revoke_api_key))
        .route("/api/access/permissions/:user_id", get(crate::api::handlers::auth::get_permissions))
        // I-FR-25: Rate limiting
        .route("/api/ratelimit/config", get(crate::api::handlers::auth::get_rate_limit_config))
        .route("/api/ratelimit/config", put(crate::api::handlers::auth::update_rate_limit_config))
        .with_state(db_pool)
}