// Workflow model
// I-FR-26: Workflow visibility and status display
// I-FR-32: Technical UI for AI workflow creation
// I-FR-33: Preprocessing AI workflow

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProcessingJob {
    pub job_id: Uuid,
    pub asset_uuid: Uuid,
    pub workflow_name: String,
    pub status: JobStatus,
    pub progress_percentage: i32,
    pub capabilities_completed: Vec<String>,
    pub capabilities_failed: Vec<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub retry_count: i32,
    pub retry_config: Option<serde_json::Value>, // I-FR-16: Retry configuration
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_status", rename_all = "UPPERCASE")]
pub enum JobStatus {
    Queued,
    Processing,
    Completed,
    Failed,
    Retrying,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub workflow_id: Uuid,
    pub workflow_name: String,
    pub description: String,
    pub step_functions_arn: String,
    pub preprocessing_logic: serde_json::Value, // I-FR-33: Business logic routing
    pub ai_capabilities: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub is_active: bool,
}
