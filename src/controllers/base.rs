// Base controller traits and common functionality
// I-FR-03: Asynchronous execution
// I-FR-04: Parallel execution
// I-FR-05: Action record generation
// I-FR-08: Scalability
// I-FR-09: Real-time monitoring
// I-FR-12: Configurable logging
// I-FR-13: Rollback mechanisms
// I-FR-16: Retry mechanisms

use crate::models::action_record::ActionRecord;
use crate::models::asset::Asset;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{error, info, warn};

#[async_trait]
pub trait Controller: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    
    // I-FR-01: Sync with configurable interval
    async fn sync(&self) -> Result<SyncResult>;
    
    // I-FR-02: Hash-based deduplication
    async fn check_duplicate(&self, file_hash: &str) -> Result<Option<Asset>>;
    
    // I-FR-05: Generate action records
    async fn log_action(&self, action: ActionRecord) -> Result<()>;
    
    // I-FR-13: Rollback support
    async fn rollback(&self, asset_uuid: uuid::Uuid, version: i32) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct SyncResult {
    pub assets_processed: usize,
    pub assets_created: usize,
    pub assets_updated: usize,
    pub assets_skipped: usize,
    pub errors: Vec<String>,
}

// I-FR-16: Retry mechanism with exponential backoff
pub async fn retry_with_backoff<F, T>(
    operation: F,
    max_attempts: u32,
    initial_delay: std::time::Duration,
) -> Result<T>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>> + Send + Sync,
{
    let mut delay = initial_delay;
    
    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(result) => {
                if attempt > 1 {
                    info!("Operation succeeded after {} attempts", attempt);
                }
                return Ok(result);
            }
            Err(e) => {
                if attempt == max_attempts {
                    error!("Operation failed after {} attempts: {}", max_attempts, e);
                    return Err(e);
                }
                warn!("Attempt {} failed: {}. Retrying in {:?}...", attempt, e, delay);
                tokio::time::sleep(delay).await;
                delay = delay * 2; // Exponential backoff
            }
        }
    }
    
    unreachable!()
}
