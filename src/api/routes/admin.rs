// Admin routes
// I-FR-09: Real-time monitoring
// I-FR-05: Action records
// I-FR-13: Rollback
// I-FR-01: Sync interval configuration
// I-FR-12: Logging level configuration
// I-FR-16: Retry configuration
// I-FR-15: Lifecycle management

use axum::{
    routing::{get, post, put},
    Router,
};
use crate::db::DbPool;

pub fn create_admin_routes(db_pool: DbPool) -> Router {
    Router::new()
        // I-FR-09: Controller monitoring
        .route("/api/controllers/status", get(crate::api::handlers::admin::get_controller_status))
        .route("/api/controllers/metrics", get(crate::api::handlers::admin::get_controller_metrics))
        // I-FR-05: Action records
        .route("/api/audit/actions", get(crate::api::handlers::admin::get_action_records))
        .route("/api/audit/actions/:asset_id", get(crate::api::handlers::admin::get_asset_actions))
        // I-FR-13: Rollback
        .route("/api/rollback/:asset_id/:version_id", post(crate::api::handlers::admin::rollback_asset))
        // Configuration endpoints
        .route("/api/config/sync-interval", get(crate::api::handlers::admin::get_sync_interval))
        .route("/api/config/sync-interval", put(crate::api::handlers::admin::update_sync_interval))
        .route("/api/config/logging-level", put(crate::api::handlers::admin::update_logging_level))
        .route("/api/config/retry", put(crate::api::handlers::admin::update_retry_config))
        // I-FR-15: Lifecycle management
        .route("/api/lifecycle/rules", get(crate::api::handlers::admin::get_lifecycle_rules))
        .route("/api/lifecycle/rules", post(crate::api::handlers::admin::create_lifecycle_rule))
        .with_state(db_pool)
}