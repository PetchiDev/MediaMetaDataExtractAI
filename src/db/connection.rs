// Database connection pool
// Supports both local PostgreSQL and AWS RDS
use sqlx::{PgPool, postgres::PgPoolOptions, postgres::PgConnectOptions};
use crate::config::AppConfig;
use anyhow::Result;
use std::sync::Arc;
use std::str::FromStr;

pub type DbPool = Arc<PgPool>;

pub async fn create_pool(config: &AppConfig) -> Result<DbPool> {
    let mut connect_options = PgConnectOptions::from_str(&config.database.postgres_url)?;
    
    // For AWS RDS, enable SSL
    if config.database.use_ssl {
        connect_options = connect_options.ssl_mode(sqlx::postgres::PgSslMode::Require);
    }
    
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect_with(connect_options)
        .await?;
    
    Ok(Arc::new(pool))
}

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
