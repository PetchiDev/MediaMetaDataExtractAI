// Action records repository
// I-FR-05: Generate action records for traceability

use crate::db::DbPool;
use crate::models::action_record::ActionRecord;
use anyhow::Result;
use uuid::Uuid;

pub struct ActionRepository;

impl ActionRepository {
    pub async fn create(pool: &DbPool, action: &ActionRecord) -> Result<Uuid> {
        let record_id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO action_records (
                record_id, asset_uuid, action_type, direction, controller_name,
                controller_version, source_system, destination_system, status,
                timestamp, metadata, user_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING record_id
            "#
        )
        .bind(&action.record_id)
        .bind(&action.asset_uuid)
        .bind(&action.action_type)
        .bind(&action.direction)
        .bind(&action.controller_name)
        .bind(&action.controller_version)
        .bind(&action.source_system)
        .bind(&action.destination_system)
        .bind(&action.status)
        .bind(action.timestamp)
        .bind(&action.metadata)
        .bind(&action.user_id)
        .fetch_one(pool.as_ref())
        .await?;

        Ok(record_id)
    }

    pub async fn get_by_asset(pool: &DbPool, asset_uuid: Uuid) -> Result<Vec<ActionRecord>> {
        let records = sqlx::query_as::<_, ActionRecord>(
            "SELECT * FROM action_records WHERE asset_uuid = $1 ORDER BY timestamp DESC"
        )
        .bind(asset_uuid)
        .fetch_all(pool.as_ref())
        .await?;

        Ok(records)
    }

    pub async fn get_recent(
        pool: &DbPool,
        limit: i64,
    ) -> Result<Vec<ActionRecord>> {
        let records = sqlx::query_as::<_, ActionRecord>(
            "SELECT * FROM action_records ORDER BY timestamp DESC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(pool.as_ref())
        .await?;

        Ok(records)
    }

    pub async fn get_by_controller(
        pool: &DbPool,
        controller_name: &str,
        limit: i64,
    ) -> Result<Vec<ActionRecord>> {
        let records = sqlx::query_as::<_, ActionRecord>(
            "SELECT * FROM action_records WHERE controller_name = $1 ORDER BY timestamp DESC LIMIT $2"
        )
        .bind(controller_name)
        .bind(limit)
        .fetch_all(pool.as_ref())
        .await?;

        Ok(records)
    }
}
