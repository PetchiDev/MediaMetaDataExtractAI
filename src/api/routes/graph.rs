// Graph search routes
// I-FR-20: Graph indexing
// I-FR-22: User friendly search and graph exploration

use axum::{
    routing::{get, post},
    Router,
};
use crate::db::DbPool;

pub fn create_graph_routes(db_pool: DbPool) -> Router {
    Router::new()
        // I-FR-22: Graph-based search
        .route("/api/graph/search", post(crate::api::handlers::graph::search))
        .route("/api/graph/relationships", post(crate::api::handlers::graph::get_relationships))
        .with_state(db_pool)
}