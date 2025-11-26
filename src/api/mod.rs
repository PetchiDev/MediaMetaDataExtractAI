// API routes and handlers
// Maps to all API requirements (I-FR-23 through I-FR-33)

mod routes;
mod handlers;
mod middleware;
mod openapi;

use axum::Router;
use crate::db::DbPool;

pub async fn create_router(db_pool: DbPool) -> anyhow::Result<Router> {
    use tower::ServiceBuilder;
    use tower_http::{cors::CorsLayer, trace::TraceLayer};
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;
    use axum::middleware;
    use std::sync::Arc;
    
    let openapi = openapi::ApiDoc::openapi();
    
    // Public routes (no auth required)
    let public_routes = Router::new()
        .merge(routes::auth::create_auth_routes(db_pool.clone()))
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", openapi.clone())
        );
    
    // Protected routes (require authentication)
    // Apply auth middleware to all protected routes
    let db_pool_for_middleware = db_pool.clone();
    let protected_routes = Router::new()
        .merge(routes::media::create_media_routes(db_pool.clone()))
        .merge(routes::metadata::create_metadata_routes(db_pool.clone()))
        .merge(routes::workflow::create_workflow_routes(db_pool.clone()))
        .merge(routes::graph::create_graph_routes(db_pool.clone()))
        .merge(routes::admin::create_admin_routes(db_pool.clone()))
        .layer(
            axum::middleware::from_fn(move |request: axum::extract::Request, next: axum::middleware::Next| {
                let db_pool = db_pool_for_middleware.clone();
                async move {
                    use crate::middleware::auth::authenticate;
                    use std::sync::Arc;
                    
                    // Extract headers
                    let headers = request.headers().clone();
                    
                    // Create a new request with state in extensions
                    let mut req = request;
                    req.extensions_mut().insert(Arc::new(db_pool));
                    
                    // Call authenticate middleware
                    authenticate(headers, req, next).await
                }
            })
        );
    
    let router = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    Ok(router)
}
