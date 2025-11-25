// Metadata routes
// I-FR-24: Metadata access and user facing API
// I-FR-27: Metadata editing and unified input
// I-FR-19: Conflict resolution

use axum::{
    routing::{get, put, post},
    Router,
};
use crate::db::DbPool;

pub fn create_metadata_routes(db_pool: DbPool) -> Router {
    Router::new()
        // I-FR-24: Query metadata
        .route("/api/metadata/:asset_id", get(crate::api::handlers::metadata::get_metadata))
        // I-FR-27: Edit metadata
        .route("/api/metadata/:asset_id", put(crate::api::handlers::metadata::update_metadata))
        // I-FR-19: Conflict resolution
        .route("/api/metadata/:asset_id/resolve-conflict", post(crate::api::handlers::metadata::resolve_conflict))
        .with_state(db_pool)
}