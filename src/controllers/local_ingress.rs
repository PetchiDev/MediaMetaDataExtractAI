// Local File Ingress Controller
// For local file system ingestion (testing without external APIs)

use crate::controllers::base::{Controller, SyncResult};
use crate::models::action_record::{ActionRecord, ActionStatus, ActionType, Direction};
use crate::models::asset::{Asset, AssetStatus, AssetType, SourceSystem};
use crate::db::DbPool;
use crate::db::repositories::{asset_repository::AssetRepository, action_repository::ActionRepository};
use crate::utils::hash;
use crate::services::{local_storage::LocalStorageService, ai_processing::AIProcessingService};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;
use std::path::Path;
use std::fs;
use chrono::Utc;

pub struct LocalFileIngressController {
    name: String,
    version: String,
    watch_directory: String,
    db_pool: DbPool,
}

impl LocalFileIngressController {
    pub fn new(watch_directory: String, db_pool: DbPool) -> Self {
        Self {
            name: "LocalFileIngressController".to_string(),
            version: "v1.0.0".to_string(),
            watch_directory,
            db_pool,
        }
    }
    
    /// Scan directory for new files and ingest them
    pub async fn scan_and_ingest(&self) -> Result<SyncResult> {
        let mut ingested = 0;
        let mut skipped = 0;
        let mut errors: Vec<String> = Vec::new();
        
        // Read directory
        let entries = fs::read_dir(&self.watch_directory)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                match self.ingest_file(&path).await {
                    Ok(true) => ingested += 1,
                    Ok(false) => skipped += 1,
                    Err(e) => {
                        let error_msg = format!("Error ingesting {:?}: {}", path, e);
                        eprintln!("{}", error_msg);
                        errors.push(error_msg);
                    }
                }
            }
        }
        
        Ok(SyncResult {
            assets_processed: ingested,
            assets_created: ingested,
            assets_updated: 0,
            assets_skipped: skipped,
            errors: errors,
        })
    }
    
    async fn ingest_file(&self, file_path: &Path) -> Result<bool> {
        // Read file
        let file_data = fs::read(file_path)?;
        
        // Calculate hash
        let file_hash = hash::calculate_file_hash_async(&file_data).await?;
        
        // Check for duplicate
        if let Ok(Some(_existing)) = AssetRepository::find_by_hash(&self.db_pool, &file_hash).await {
            return Ok(false); // Skip duplicate
        }
        
        // Determine asset type from extension
        let asset_type = if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "mp4" | "avi" | "mov" => AssetType::Video,
                "mp3" | "wav" | "m4a" => AssetType::Audio,
                "png" | "jpg" | "jpeg" | "gif" => AssetType::Image,
                "pdf" | "txt" | "doc" | "docx" | "html" | "json" => AssetType::Text,
                _ => AssetType::Text,
            }
        } else {
            AssetType::Text
        };
        
        // Move file to storage
        let filename = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        let local_storage = LocalStorageService::new(None);
        let storage_path = local_storage.save_file("ingress", filename, file_data).await?;
        
        // Create asset
        let asset_uuid = Uuid::new_v4();
        let asset = Asset {
            uuid: asset_uuid,
            asset_type: asset_type.clone(),
            asset_name: filename.to_string(),
            source_system: SourceSystem::UserUpload,
            source_id: Some(file_path.to_string_lossy().to_string()),
            file_path: storage_path.clone(),
            file_hash,
            file_size: fs::metadata(file_path)?.len() as i64,
            duration: None,
            format: file_path.extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_uppercase())
                .unwrap_or_else(|| "UNKNOWN".to_string()),
            status: AssetStatus::Queued,
            version: 1,
            version_id: Uuid::new_v4(),
            enriched_metadata: serde_json::json!({}),
            operational_tags: None,
            created_at: Utc::now(),
            updated_at: None,
            processing_completed_at: None,
            uploaded_by: None,
        };
        
        AssetRepository::create(&self.db_pool, &asset).await?;
        
        // Log action
        let action = ActionRecord {
            record_id: Uuid::new_v4(),
            asset_uuid: Some(asset_uuid),
            action_type: ActionType::Ingress,
            direction: Direction::Inbound,
            controller_name: self.name.clone(),
            controller_version: self.version.clone(),
            source_system: Some("LOCAL_FILE_SYSTEM".to_string()),
            destination_system: Some("AI_PROCESSING_PIPELINE".to_string()),
            status: ActionStatus::Success,
            timestamp: Utc::now(),
            metadata: Some(serde_json::json!({
                "file_path": file_path.to_string_lossy(),
                "asset_type": format!("{:?}", asset_type.clone()),
            })),
            user_id: None,
        };
        
        ActionRepository::create(&self.db_pool, &action).await?;
        
        // Trigger AI processing in background
        let db_pool_clone = self.db_pool.clone();
        let asset_uuid_clone = asset_uuid;
        let storage_path_clone = storage_path.clone();
        let asset_type_str = format!("{:?}", asset_type.clone());
        tokio::spawn(async move {
            // Update status to processing
            AssetRepository::update_status(
                &db_pool_clone, 
                asset_uuid_clone, 
                AssetStatus::Processing,
                None
            ).await.ok();
            
            // Process with AI
            let ai_results = AIProcessingService::process_asset(
                &storage_path_clone,
                &asset_type_str,
            ).await;
            
            if let Ok(enriched_metadata) = ai_results {
                // Get current asset
                if let Ok(Some(asset)) = AssetRepository::get_by_uuid(&db_pool_clone, asset_uuid_clone).await {
                    // Merge AI results
                    let mut metadata = asset.enriched_metadata.clone();
                    if let Some(obj) = enriched_metadata.as_object() {
                        for (key, value) in obj {
                            metadata[key] = value.clone();
                        }
                    }
                    
                    // Update asset
                    AssetRepository::update_metadata(
                        &db_pool_clone,
                        asset_uuid_clone,
                        metadata,
                        asset.version,
                        asset.version_id,
                        Uuid::new_v4(),
                    ).await.ok();
                    
                    // Update status with completion timestamp
                    AssetRepository::update_status(
                        &db_pool_clone, 
                        asset_uuid_clone, 
                        AssetStatus::Processed,
                        Some(Utc::now())
                    ).await.ok();
                }
            }
        });
        
        Ok(true)
    }
}

#[async_trait]
impl Controller for LocalFileIngressController {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    async fn sync(&self) -> Result<SyncResult> {
        self.scan_and_ingest().await
    }
    
    async fn check_duplicate(&self, file_hash: &str) -> Result<Option<Asset>> {
        AssetRepository::find_by_hash(&self.db_pool, file_hash).await
    }
    
    async fn log_action(&self, action: ActionRecord) -> Result<()> {
        ActionRepository::create(&self.db_pool, &action).await.map(|_| ())
    }
    
    async fn rollback(&self, _asset_uuid: Uuid, _version: i32) -> Result<()> {
        todo!("Implement rollback for local files")
    }
}

