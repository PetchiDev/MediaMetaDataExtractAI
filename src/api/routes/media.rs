// Media routes
// I-FR-29: Media submission and ingestion endpoint
// I-FR-31: Media upload and ingestion

use axum::{
    routing::{get, post},
    Router,
};
use crate::db::DbPool;

pub fn create_media_routes(db_pool: DbPool) -> Router {
    Router::new()
        // I-FR-29: API-based media submission (technical users)
        .route("/api/media/submit", post(crate::api::handlers::media::submit_media))
        // I-FR-31: Manual upload via UI (naive users)
        .route("/api/media/upload", post(crate::api::handlers::media::upload_media))
        .route("/api/media/:asset_id", get(crate::api::handlers::media::get_media))
        .with_state(db_pool)
}