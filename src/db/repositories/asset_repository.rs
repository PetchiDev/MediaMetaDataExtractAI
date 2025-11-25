// Asset repository
// I-FR-02: Hash-based deduplication
// I-FR-18: Version control
// I-FR-19: Conflict detection

use crate::db::DbPool;
use crate::models::asset::{Asset, AssetStatus, AssetType, AssetVersion, SourceSystem};
use anyhow::Result;
use sqlx::FromRow;
use uuid::Uuid;
use chrono::Utc;

pub struct AssetRepository;

impl AssetRepository {
    // I-FR-02: Check for duplicate by hash
    pub async fn find_by_hash(pool: &DbPool, file_hash: &str) -> Result<Option<Asset>> {
        let asset = sqlx::query_as::<_, Asset>(
            "SELECT * FROM assets WHERE file_hash = $1 LIMIT 1"
        )
        .bind(file_hash)
        .fetch_optional(pool.as_ref())
        .await?;
        
        Ok(asset)
    }

    pub async fn create(pool: &DbPool, asset: &Asset) -> Result<Uuid> {
        let uuid = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO assets (
                uuid, asset_type, asset_name, source_system, source_id,
                file_path, file_hash, file_size, duration, format, status,
                version, version_id, enriched_metadata, operational_tags,
                uploaded_by, created_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            ) RETURNING uuid
            "#
        )
        .bind(&asset.uuid)
        .bind(&asset.asset_type)
        .bind(&asset.asset_name)
        .bind(&asset.source_system)
        .bind(&asset.source_id)
        .bind(&asset.file_path)
        .bind(&asset.file_hash)
        .bind(asset.file_size)
        .bind(asset.duration)
        .bind(&asset.format)
        .bind(&asset.status)
        .bind(asset.version)
        .bind(&asset.version_id)
        .bind(&asset.enriched_metadata)
        .bind(&asset.operational_tags)
        .bind(&asset.uploaded_by)
        .bind(asset.created_at)
        .fetch_one(pool.as_ref())
        .await?;

        Ok(uuid)
    }

    pub async fn get_by_uuid(pool: &DbPool, uuid: Uuid) -> Result<Option<Asset>> {
        let asset = sqlx::query_as::<_, Asset>(
            "SELECT * FROM assets WHERE uuid = $1"
        )
        .bind(uuid)
        .fetch_optional(pool.as_ref())
        .await?;

        Ok(asset)
    }

    // I-FR-18: Create new version
    pub async fn create_version(
        pool: &DbPool,
        asset_uuid: Uuid,
        version: i32,
        version_id: Uuid,
        metadata_snapshot: serde_json::Value,
        created_by: Uuid,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO asset_versions (
                asset_uuid, version, version_id, metadata_snapshot, created_by, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(asset_uuid)
        .bind(version)
        .bind(version_id)
        .bind(metadata_snapshot)
        .bind(created_by)
        .bind(Utc::now())
        .execute(pool.as_ref())
        .await?;

        Ok(())
    }

    // I-FR-19: Check for conflicts (compare version_id)
    pub async fn check_version_conflict(
        pool: &DbPool,
        asset_uuid: Uuid,
        provided_version_id: Uuid,
    ) -> Result<bool> {
        let current_version_id: Option<Uuid> = sqlx::query_scalar(
            "SELECT version_id FROM assets WHERE uuid = $1"
        )
        .bind(asset_uuid)
        .fetch_optional(pool.as_ref())
        .await?;

        match current_version_id {
            Some(current) => Ok(current != provided_version_id),
            None => Ok(true), // Asset doesn't exist, treat as conflict
        }
    }

    pub async fn update_metadata(
        pool: &DbPool,
        asset_uuid: Uuid,
        enriched_metadata: serde_json::Value,
        version: i32,
        version_id: Uuid,
        updated_by: Uuid,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE assets
            SET enriched_metadata = $1, version = $2, version_id = $3,
                updated_at = $4, updated_by = $5
            WHERE uuid = $6
            "#
        )
        .bind(enriched_metadata)
        .bind(version)
        .bind(version_id)
        .bind(Utc::now())
        .bind(updated_by)
        .bind(asset_uuid)
        .execute(pool.as_ref())
        .await?;

        Ok(())
    }

    pub async fn update_status(
        pool: &DbPool,
        asset_uuid: Uuid,
        status: AssetStatus,
        processing_completed_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE assets
            SET status = $1, processing_completed_at = $2
            WHERE uuid = $3
            "#
        )
        .bind(status)
        .bind(processing_completed_at)
        .bind(asset_uuid)
        .execute(pool.as_ref())
        .await?;

        Ok(())
    }

    // I-FR-13: Rollback to previous version
    pub async fn rollback_to_version(
        pool: &DbPool,
        asset_uuid: Uuid,
        version: i32,
    ) -> Result<()> {
        // Get version snapshot
        let version_snapshot: serde_json::Value = sqlx::query_scalar(
            "SELECT metadata_snapshot FROM asset_versions WHERE asset_uuid = $1 AND version = $2"
        )
        .bind(asset_uuid)
        .bind(version)
        .fetch_one(pool.as_ref())
        .await?;

        // Restore asset metadata
        sqlx::query(
            r#"
            UPDATE assets
            SET enriched_metadata = $1, version = $2, version_id = uuid_generate_v4()
            WHERE uuid = $3
            "#
        )
        .bind(version_snapshot)
        .bind(version)
        .bind(asset_uuid)
        .execute(pool.as_ref())
        .await?;

        Ok(())
    }
}
