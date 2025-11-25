// API routes and handlers
// Maps to all API requirements (I-FR-23 through I-FR-33)

mod routes;
mod handlers;
mod middleware;

use axum::Router;
use crate::db::DbPool;

pub async fn create_router(db_pool: DbPool) -> anyhow::Result<Router> {
    use tower::ServiceBuilder;
    use tower_http::{cors::CorsLayer, trace::TraceLayer};
    
    let router = Router::new()
        .merge(routes::media::create_media_routes(db_pool.clone()))
        .merge(routes::metadata::create_metadata_routes(db_pool.clone()))
        .merge(routes::workflow::create_workflow_routes(db_pool.clone()))
        .merge(routes::graph::create_graph_routes(db_pool.clone()))
        .merge(routes::admin::create_admin_routes(db_pool.clone()))
        .merge(routes::auth::create_auth_routes(db_pool.clone()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    Ok(router)
}
