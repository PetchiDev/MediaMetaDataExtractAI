// Workflow repository
// I-FR-26: Workflow status tracking
// I-FR-32: Workflow management

use crate::db::DbPool;
use crate::models::workflow::{ProcessingJob, WorkflowDefinition};
use anyhow::Result;
use sqlx::Row;
use uuid::Uuid;

pub struct WorkflowRepository;

impl WorkflowRepository {
    pub async fn create_job(pool: &DbPool, job: &ProcessingJob) -> Result<Uuid> {
        let job_id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO processing_jobs (
                job_id, asset_uuid, workflow_name, status, progress_percentage,
                capabilities_completed, capabilities_failed, error_message,
                created_at, estimated_completion, retry_count, retry_config
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING job_id
            "#
        )
        .bind(&job.job_id)
        .bind(&job.asset_uuid)
        .bind(&job.workflow_name)
        .bind(&job.status)
        .bind(job.progress_percentage)
        .bind(serde_json::to_value(&job.capabilities_completed)?)
        .bind(serde_json::to_value(&job.capabilities_failed)?)
        .bind(&job.error_message)
        .bind(job.created_at)
        .bind(job.estimated_completion)
        .bind(job.retry_count)
        .bind(job.retry_config.as_ref().map(serde_json::to_value).transpose()?)
        .fetch_one(pool.as_ref())
        .await?;

        Ok(job_id)
    }

    pub async fn get_job_by_asset(pool: &DbPool, asset_uuid: Uuid) -> Result<Option<ProcessingJob>> {
        let row = sqlx::query(
            r#"
            SELECT job_id, asset_uuid, workflow_name, status, progress_percentage,
                   capabilities_completed, capabilities_failed, error_message,
                   created_at, started_at, completed_at, estimated_completion,
                   retry_count, retry_config
            FROM processing_jobs WHERE asset_uuid = $1 ORDER BY created_at DESC LIMIT 1
            "#
        )
        .bind(asset_uuid)
        .fetch_optional(pool.as_ref())
        .await?;

        if let Some(row) = row {
            Ok(Some(ProcessingJob {
                job_id: row.get("job_id"),
                asset_uuid: row.get("asset_uuid"),
                workflow_name: row.get("workflow_name"),
                status: row.get("status"),
                progress_percentage: row.get("progress_percentage"),
                capabilities_completed: serde_json::from_value(row.get("capabilities_completed"))?,
                capabilities_failed: serde_json::from_value(row.get("capabilities_failed"))?,
                error_message: row.get("error_message"),
                created_at: row.get("created_at"),
                started_at: row.get("started_at"),
                completed_at: row.get("completed_at"),
                estimated_completion: row.get("estimated_completion"),
                retry_count: row.get("retry_count"),
                retry_config: row.get::<Option<serde_json::Value>, _>("retry_config"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_job(pool: &DbPool, job_id: Uuid) -> Result<Option<ProcessingJob>> {
        let row = sqlx::query(
            r#"
            SELECT job_id, asset_uuid, workflow_name, status, progress_percentage,
                   capabilities_completed, capabilities_failed, error_message,
                   created_at, started_at, completed_at, estimated_completion,
                   retry_count, retry_config
            FROM processing_jobs WHERE job_id = $1
            "#
        )
        .bind(job_id)
        .fetch_optional(pool.as_ref())
        .await?;

        if let Some(row) = row {
            Ok(Some(ProcessingJob {
                job_id: row.get("job_id"),
                asset_uuid: row.get("asset_uuid"),
                workflow_name: row.get("workflow_name"),
                status: row.get("status"),
                progress_percentage: row.get("progress_percentage"),
                capabilities_completed: serde_json::from_value(row.get("capabilities_completed"))?,
                capabilities_failed: serde_json::from_value(row.get("capabilities_failed"))?,
                error_message: row.get("error_message"),
                created_at: row.get("created_at"),
                started_at: row.get("started_at"),
                completed_at: row.get("completed_at"),
                estimated_completion: row.get("estimated_completion"),
                retry_count: row.get("retry_count"),
                retry_config: row.get::<Option<serde_json::Value>, _>("retry_config"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update_job_status(
        pool: &DbPool,
        job_id: Uuid,
        status: crate::models::workflow::JobStatus,
        progress: Option<i32>,
        error_message: Option<String>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE processing_jobs
            SET status = $1, progress_percentage = COALESCE($2, progress_percentage),
                error_message = COALESCE($3, error_message),
                completed_at = CASE WHEN $1 = 'COMPLETED' THEN NOW() ELSE completed_at END
            WHERE job_id = $4
            "#
        )
        .bind(status)
        .bind(progress)
        .bind(error_message)
        .bind(job_id)
        .execute(pool.as_ref())
        .await?;

        Ok(())
    }

    pub async fn create_workflow_definition(
        pool: &DbPool,
        workflow: &WorkflowDefinition,
    ) -> Result<Uuid> {
        let workflow_id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO workflow_definitions (
                workflow_id, workflow_name, description, step_functions_arn,
                preprocessing_logic, ai_capabilities, created_at, created_by, is_active
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING workflow_id
            "#
        )
        .bind(&workflow.workflow_id)
        .bind(&workflow.workflow_name)
        .bind(&workflow.description)
        .bind(&workflow.step_functions_arn)
        .bind(&workflow.preprocessing_logic)
        .bind(serde_json::to_value(&workflow.ai_capabilities)?)
        .bind(workflow.created_at)
        .bind(&workflow.created_by)
        .bind(workflow.is_active)
        .fetch_one(pool.as_ref())
        .await?;

        Ok(workflow_id)
    }
}
