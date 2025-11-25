// Configuration module
// I-FR-01: Configurable sync intervals
// I-FR-12: Configurable logging levels
// I-FR-16: User-configurable retry mechanisms

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub aws: AwsConfig,
    pub sync: SyncConfig,
    pub logging: LoggingConfig,
    pub retry: RetryConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub postgres_url: String,
    pub neptune_endpoint: Option<String>,
    pub use_ssl: bool, // For AWS RDS
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    pub region: String,
    pub s3_bucket_staging: String,
    pub s3_bucket_processed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    // I-FR-01: Configurable sync interval (default 15 minutes)
    pub default_interval_minutes: u64,
    pub brightcove_interval_minutes: Option<u64>,
    pub cloudinary_interval_minutes: Option<u64>,
    pub omnystudio_interval_minutes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    // I-FR-12: Configurable logging levels (critical, error, warning, info)
    pub level: String,
    pub controller_logging: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    // I-FR-16: User-configurable retry mechanisms
    pub max_attempts: u32,
    pub initial_interval_seconds: u64,
    pub max_interval_seconds: u64,
    pub backoff_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub sso_provider: String,
    pub rate_limit_per_minute: u32,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        // Load from environment variables or config file
        Ok(AppConfig {
            database: DatabaseConfig {
                postgres_url: std::env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgresql://postgres:password%401@localhost:5432/MediaAI".to_string()),
                neptune_endpoint: std::env::var("NEPTUNE_ENDPOINT").ok(),
                use_ssl: std::env::var("DATABASE_USE_SSL")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse::<bool>()
                    .unwrap_or(false),
                max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "20".to_string())
                    .parse::<u32>()
                    .unwrap_or(20),
            },
            aws: AwsConfig {
                region: std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
                s3_bucket_staging: std::env::var("S3_BUCKET_STAGING")
                    .unwrap_or_else(|_| "mediacorp-ai-ingress-staging".to_string()),
                s3_bucket_processed: std::env::var("S3_BUCKET_PROCESSED")
                    .unwrap_or_else(|_| "mediacorp-ai-processed".to_string()),
            },
            sync: SyncConfig {
                default_interval_minutes: 15, // I-FR-01: Default 15 minutes
                brightcove_interval_minutes: None,
                cloudinary_interval_minutes: None,
                omnystudio_interval_minutes: None,
            },
            logging: LoggingConfig {
                level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
                controller_logging: std::collections::HashMap::new(),
            },
            retry: RetryConfig {
                max_attempts: 3,
                initial_interval_seconds: 5,
                max_interval_seconds: 300,
                backoff_multiplier: 2.0,
            },
            security: SecurityConfig {
                jwt_secret: std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "change-me-in-production".to_string()),
                sso_provider: std::env::var("SSO_PROVIDER")
                    .unwrap_or_else(|_| "mediacorp-sso".to_string()),
                rate_limit_per_minute: 100, // I-FR-25: Rate limiting
            },
        })
    }
}
