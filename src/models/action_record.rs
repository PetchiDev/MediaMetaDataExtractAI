// Action Record model
// I-FR-05: Generate action records for traceability

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ActionRecord {
    pub record_id: Uuid,
    pub asset_uuid: Option<Uuid>,
    pub action_type: ActionType,
    pub direction: Direction,
    pub controller_name: String,
    pub controller_version: String,
    pub source_system: Option<String>,
    pub destination_system: Option<String>,
    pub status: ActionStatus,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "action_type", rename_all = "UPPERCASE")]
pub enum ActionType {
    Ingress,
    Egress,
    UserUpload,
    ApiSubmission,
    MetadataUpdate,
    ConflictResolved,
    JobRetry,
    Rollback,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "direction", rename_all = "UPPERCASE")]
pub enum Direction {
    Inbound,
    Outbound,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "action_status", rename_all = "UPPERCASE")]
pub enum ActionStatus {
    Success,
    Failed,
    InProgress,
    Initiated,
}
