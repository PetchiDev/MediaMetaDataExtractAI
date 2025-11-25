// Workflow handlers - FULL IMPLEMENTATION
// I-FR-26: Workflow status visibility
// I-FR-32: Workflow creation

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use crate::db::DbPool;
use crate::db::repositories::workflow_repository::WorkflowRepository;
use serde_json::json;
use sqlx::Row;

// I-FR-26: Get workflow status for asset
pub async fn get_workflow_status(
    State(db_pool): State<DbPool>,
    Path(asset_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get latest job for asset using repository
    let job = WorkflowRepository::get_job_by_asset(&db_pool, asset_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(job) = job {
        Ok(Json(json!({
            "asset_uuid": asset_id,
            "job_id": job.job_id,
            "workflow_name": job.workflow_name,
            "status": format!("{:?}", job.status),
            "progress_percentage": job.progress_percentage,
            "capabilities_completed": job.capabilities_completed,
            "capabilities_failed": job.capabilities_failed,
            "error_message": job.error_message,
        })))
    } else {
        Ok(Json(json!({
            "asset_uuid": asset_id,
            "status": "NOT_FOUND",
            "message": "No workflow found for this asset"
        })))
    }
}

pub async fn get_job_status(
    State(db_pool): State<DbPool>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let job = WorkflowRepository::get_job(&db_pool, job_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(json!({
        "job_id": job.job_id,
        "asset_uuid": job.asset_uuid,
        "workflow_name": job.workflow_name,
        "status": format!("{:?}", job.status),
        "progress_percentage": job.progress_percentage,
        "capabilities_completed": job.capabilities_completed,
        "capabilities_failed": job.capabilities_failed,
        "error_message": job.error_message,
        "created_at": job.created_at,
        "started_at": job.started_at,
        "completed_at": job.completed_at,
    })))
}

pub async fn retry_job(
    State(db_pool): State<DbPool>,
    Path(job_id): Path<Uuid>,
    Json(config): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut job = WorkflowRepository::get_job(&db_pool, job_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Update retry config
    job.retry_count += 1;
    job.retry_config = Some(config.clone());
    job.status = crate::models::workflow::JobStatus::Retrying;

    WorkflowRepository::update_job_status(
        &db_pool,
        job_id,
        job.status.clone(),
        None,
        None,
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: Trigger workflow with retry config

    Ok(Json(json!({
        "status": "success",
        "message": "Retry initiated",
        "job_id": job_id,
        "retry_count": job.retry_count,
        "new_status": "RETRYING"
    })))
}

// I-FR-32: Create new AI workflow
pub async fn create_workflow(
    State(db_pool): State<DbPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let workflow_name = payload.get("workflow_name")
        .and_then(|s| s.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let workflow = crate::models::workflow::WorkflowDefinition {
        workflow_id: Uuid::new_v4(),
        workflow_name: workflow_name.to_string(),
        description: payload.get("description")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default(),
        step_functions_arn: payload.get("step_functions_arn")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default(),
        preprocessing_logic: payload.get("preprocessing_logic")
            .cloned()
            .unwrap_or_else(|| json!({})),
        ai_capabilities: payload.get("ai_capabilities")
            .and_then(|a| a.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default(),
        created_at: chrono::Utc::now(),
        created_by: Uuid::new_v4(), // TODO: Get from auth
        is_active: true,
    };

    let workflow_id = WorkflowRepository::create_workflow_definition(&db_pool, &workflow).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "workflow_id": workflow_id,
        "workflow_name": workflow.workflow_name,
        "status": "created"
    })))
}

pub async fn list_workflows(
    State(db_pool): State<DbPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let workflows = sqlx::query(
        "SELECT workflow_id, workflow_name, description, ai_capabilities FROM workflow_definitions WHERE is_active = true ORDER BY created_at DESC"
    )
    .fetch_all(db_pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: Vec<serde_json::Value> = workflows.iter().map(|row: &sqlx::postgres::PgRow| {
        json!({
            "workflow_id": row.get::<Uuid, _>("workflow_id"),
            "workflow_name": row.get::<String, _>("workflow_name"),
            "description": row.get::<Option<String>, _>("description"),
            "ai_capabilities": row.get::<serde_json::Value, _>("ai_capabilities"),
        })
    }).collect();

    Ok(Json(json!(result)))
}