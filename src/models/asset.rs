// Asset model
// Core entity representing media assets (Video, Image, Audio, Text)
// I-FR-02: Hash-based deduplication
// I-FR-15: Lifecycle management
// I-FR-18: Version control

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Asset {
    pub uuid: Uuid,
    pub asset_type: AssetType,
    pub asset_name: String,
    pub source_system: SourceSystem,
    pub source_id: Option<String>,
    pub file_path: String,
    pub file_hash: String, // I-FR-02: Hash for deduplication
    pub file_size: i64,
    pub duration: Option<i32>, // For video/audio
    pub format: String,
    pub status: AssetStatus,
    pub version: i32, // I-FR-18: Version control
    pub version_id: Uuid,
    pub enriched_metadata: serde_json::Value,
    pub operational_tags: Option<serde_json::Value>, // I-FR-30: Operational tagging
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub processing_completed_at: Option<DateTime<Utc>>,
    pub uploaded_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "asset_type", rename_all = "UPPERCASE")]
pub enum AssetType {
    Video,
    Image,
    Audio,
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "source_system", rename_all = "UPPERCASE")]
pub enum SourceSystem {
    Brightcove,
    Cloudinary,
    Omnystudio,
    OneCms,
    MissyS3,
    DaletS3,
    UserUpload,    // I-FR-31: Manual upload
    ApiSubmission, // I-FR-29: API submission
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "asset_status", rename_all = "UPPERCASE")]
pub enum AssetStatus {
    Staged,
    Queued,
    Processing,
    Processed,
    Failed,
    Archived, // I-FR-15: Lifecycle management
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetVersion {
    pub asset_uuid: Uuid,
    pub version: i32,
    pub version_id: Uuid,
    pub metadata_snapshot: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub conflict_resolved: bool, // I-FR-19: Conflict resolution
}
