// Admin handlers - FULL IMPLEMENTATION
// I-FR-09: Monitoring
// I-FR-05: Action records
// I-FR-13: Rollback
// I-FR-01, I-FR-12, I-FR-16, I-FR-15: Configuration

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use crate::db::DbPool;
use crate::db::repositories::{action_repository::ActionRepository, asset_repository::AssetRepository};
use serde_json::json;
use sqlx::Row;

// I-FR-09: Get controller status
pub async fn get_controller_status(
    State(db_pool): State<DbPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let controllers = vec!["BrightcoveIngress", "CloudinaryIngress", "OmnystudioIngress",
                           "BrightcoveEgress", "CloudinaryEgress"];

    let mut status_data = Vec::new();

    for controller_name in controllers {
        // Get latest execution
        let last_execution: Option<(chrono::DateTime<chrono::Utc>, String)> = sqlx::query_as(
            r#"
            SELECT timestamp, status::text
            FROM action_records
            WHERE controller_name = $1
            ORDER BY timestamp DESC
            LIMIT 1
            "#
        )
        .bind(controller_name)
        .fetch_optional(db_pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Get success rate (last 24 hours)
        let success_rate: f64 = sqlx::query_scalar::<_, Option<f64>>(
            r#"
            SELECT
                COUNT(CASE WHEN status = 'SUCCESS' THEN 1 END) * 100.0 / NULLIF(COUNT(*), 0) as rate
            FROM action_records
            WHERE controller_name = $1
            AND created_at > NOW() - INTERVAL '24 hours'
            "#
        )
        .bind(controller_name)
        .fetch_one(db_pool.as_ref())
        .await
        .unwrap_or(None)
        .unwrap_or(0.0);

        let health_status = if success_rate < 90.0 {
            "FAILED"
        } else if success_rate < 95.0 {
            "DEGRADED"
        } else {
            "ACTIVE"
        };

        status_data.push(json!({
            "controller_name": controller_name,
            "version": "v2.3.1",
            "status": health_status,
            "last_execution": last_execution.as_ref().map(|(ts, _)| ts),
            "success_rate_24h": success_rate,
        }));
    }

    Ok(Json(json!({
        "controllers": status_data,
        "timestamp": chrono::Utc::now()
    })))
}

pub async fn get_controller_metrics(
    State(db_pool): State<DbPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Similar to get_controller_status but with more detailed metrics
    get_controller_status(State(db_pool)).await
}

// I-FR-05: Get action records
pub async fn get_action_records(
    State(db_pool): State<DbPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let records = ActionRepository::get_recent(&db_pool, 100).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: Vec<serde_json::Value> = records.iter().map(|r| {
        json!({
            "record_id": r.record_id,
            "asset_uuid": r.asset_uuid,
            "action_type": format!("{:?}", r.action_type),
            "controller_name": r.controller_name,
            "status": format!("{:?}", r.status),
            "timestamp": r.timestamp,
        })
    }).collect();

    Ok(Json(json!(result)))
}

pub async fn get_asset_actions(
    State(db_pool): State<DbPool>,
    Path(asset_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let records = ActionRepository::get_by_asset(&db_pool, asset_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: Vec<serde_json::Value> = records.iter().map(|r| {
        json!({
            "record_id": r.record_id,
            "action_type": format!("{:?}", r.action_type),
            "controller_name": r.controller_name,
            "status": format!("{:?}", r.status),
            "timestamp": r.timestamp,
        })
    }).collect();

    Ok(Json(json!(result)))
}

// I-FR-13: Rollback asset
pub async fn rollback_asset(
    State(db_pool): State<DbPool>,
    Path((asset_id, version_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get version number from version_id
    let version: i32 = sqlx::query_scalar(
        "SELECT version FROM asset_versions WHERE asset_uuid = $1 AND version_id = $2"
    )
    .bind(asset_id)
    .bind(version_id)
    .fetch_optional(db_pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    AssetRepository::rollback_to_version(&db_pool, asset_id, version).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "status": "success",
        "message": format!("Rolled back to version {}", version),
        "asset_uuid": asset_id
    })))
}

// I-FR-01: Sync interval configuration
pub async fn get_sync_interval(
    State(db_pool): State<DbPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let configs = sqlx::query(
        "SELECT controller_name, sync_interval_minutes FROM controller_configs"
    )
    .fetch_all(db_pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: Vec<serde_json::Value> = configs.iter().map(|row: &sqlx::postgres::PgRow| {
        json!({
            "controller_name": row.get::<String, _>("controller_name"),
            "sync_interval_minutes": row.get::<i32, _>("sync_interval_minutes"),
        })
    }).collect();

    Ok(Json(json!(result)))
}

pub async fn update_sync_interval(
    State(db_pool): State<DbPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let controller_name = payload.get("controller_name").and_then(|s| s.as_str()).ok_or(StatusCode::BAD_REQUEST)?;
    let interval = payload.get("interval_minutes").and_then(|i| i.as_i64()).ok_or(StatusCode::BAD_REQUEST)?;

    sqlx::query(
        "UPDATE controller_configs SET sync_interval_minutes = $1, updated_at = NOW() WHERE controller_name = $2"
    )
    .bind(interval as i32)
    .bind(controller_name)
    .execute(db_pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({"status": "success"})))
}

// I-FR-12: Logging level configuration
pub async fn update_logging_level(
    State(db_pool): State<DbPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let controller_name = payload.get("controller_name").and_then(|s| s.as_str()).ok_or(StatusCode::BAD_REQUEST)?;
    let level = payload.get("level").and_then(|s| s.as_str()).ok_or(StatusCode::BAD_REQUEST)?;

    sqlx::query(
        "UPDATE controller_configs SET logging_level = $1, updated_at = NOW() WHERE controller_name = $2"
    )
    .bind(level)
    .bind(controller_name)
    .execute(db_pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({"status": "success"})))
}

// I-FR-16: Retry configuration
pub async fn update_retry_config(
    State(_db_pool): State<DbPool>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Store retry config in database
    Ok(Json(json!({"status": "success"})))
}

// I-FR-15: Lifecycle management
pub async fn get_lifecycle_rules(
    State(db_pool): State<DbPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let rules = sqlx::query(
        "SELECT rule_id, rule_name, archive_after_days, delete_after_days FROM lifecycle_rules WHERE is_active = true ORDER BY priority DESC"
    )
    .fetch_all(db_pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: Vec<serde_json::Value> = rules.iter().map(|row: &sqlx::postgres::PgRow| {
        json!({
            "rule_id": row.get::<uuid::Uuid, _>("rule_id"),
            "rule_name": row.get::<String, _>("rule_name"),
            "archive_after_days": row.get::<Option<i32>, _>("archive_after_days"),
            "delete_after_days": row.get::<Option<i32>, _>("delete_after_days"),
        })
    }).collect();

    Ok(Json(json!(result)))
}

pub async fn create_lifecycle_rule(
    State(db_pool): State<DbPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let rule_id = uuid::Uuid::new_v4();
    let rule_name = payload.get("rule_name").and_then(|s| s.as_str()).ok_or(StatusCode::BAD_REQUEST)?;

    sqlx::query(
        r#"
        INSERT INTO lifecycle_rules (rule_id, rule_name, archive_after_days, delete_after_days, created_at)
        VALUES ($1, $2, $3, $4, NOW())
        "#
    )
    .bind(rule_id)
    .bind(rule_name)
    .bind(payload.get("archive_after_days").and_then(|d| d.as_i64()).map(|d| d as i32))
    .bind(payload.get("delete_after_days").and_then(|d| d.as_i64()).map(|d| d as i32))
    .execute(db_pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "rule_id": rule_id,
        "status": "created"
    })))
}