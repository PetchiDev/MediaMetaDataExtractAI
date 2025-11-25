// Main entry point for AI Media Metadata Processing Platform
// Maps to all 33 Functional Requirements (I-FR-01 through I-FR-33)

mod controllers;
mod services;
mod models;
mod api;
mod config;
mod middleware;
mod utils;
mod db;
mod aws;
mod external;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize tracing (I-FR-12: Configurable Logging Levels)
    tracing_subscriber::fmt::init();

    info!("Starting AI Media Metadata Processing Platform");

    // Load configuration
    let config = config::AppConfig::load()?;

    // Initialize database connection pool
    let db_pool = db::connection::create_pool(&config).await?;
    
    // Run migrations
    db::connection::run_migrations(db_pool.as_ref()).await?;
    info!("Database migrations completed");

    // Build application router with all API endpoints
    let app = api::create_router(db_pool).await?;

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}