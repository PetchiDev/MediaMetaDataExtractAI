// Workflow routes
// I-FR-26: Workflow visibility and status display
// I-FR-32: Technical UI for AI workflow creation

use axum::{
    routing::{get, post, put},
    Router,
};
use crate::db::DbPool;

pub fn create_workflow_routes(db_pool: DbPool) -> Router {
    Router::new()
        // I-FR-26: Get workflow status
        .route("/api/workflow/status/:asset_id", get(crate::api::handlers::workflow::get_workflow_status))
        .route("/api/jobs/:job_id/status", get(crate::api::handlers::workflow::get_job_status))
        .route("/api/jobs/:job_id/retry", post(crate::api::handlers::workflow::retry_job))
        // I-FR-32: Create/manage workflows
        .route("/api/workflows", post(crate::api::handlers::workflow::create_workflow))
        .route("/api/workflows", get(crate::api::handlers::workflow::list_workflows))
        .with_state(db_pool)
}