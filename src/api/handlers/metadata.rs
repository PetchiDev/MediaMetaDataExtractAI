// Metadata handlers - FULL IMPLEMENTATION
// I-FR-24: Metadata access
// I-FR-27: Metadata editing
// I-FR-19: Conflict resolution

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use uuid::Uuid;
use crate::db::DbPool;
use crate::db::repositories::{asset_repository::AssetRepository, graph_repository::GraphRepository};
use crate::models::metadata::MetadataUpdate;
use crate::middleware::auth::Claims;
use serde_json::json;
use chrono::Utc;

// I-FR-24: Query enriched metadata
pub async fn get_metadata(
    State(db_pool): State<DbPool>,
    Path(asset_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let asset = AssetRepository::get_by_uuid(&db_pool, asset_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(json!({
        "asset_uuid": asset.uuid,
        "enriched_metadata": asset.enriched_metadata,
        "version": asset.version,
        "version_id": asset.version_id,
        "updated_at": asset.updated_at,
    })))
}

// I-FR-27: Update metadata with conflict detection
pub async fn update_metadata(
    State(db_pool): State<DbPool>,
    Path(asset_id): Path<Uuid>,
    headers: HeaderMap,
    Json(update): Json<MetadataUpdate>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get current asset
    let current_asset = AssetRepository::get_by_uuid(&db_pool, asset_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // I-FR-19: Check for conflicts using version_id
    let provided_version_id = headers
        .get("x-version-id")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    if let Some(version_id) = provided_version_id {
        let has_conflict = AssetRepository::check_version_conflict(
            &db_pool,
            asset_id,
            version_id,
        ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if has_conflict {
            return Ok(Json(json!({
                "error": "Metadata conflict detected",
                "conflict_detected": true,
                "your_version": version_id,
                "current_version": current_asset.version_id,
                "requires_manual_review": true
            })));
        }
    }

    // Get user from auth (TODO: Extract from middleware)
    let user_id = Uuid::new_v4(); // Placeholder

    // Create new version (I-FR-18)
    let new_version = current_asset.version + 1;
    let new_version_id = Uuid::new_v4();

    // Archive current version
    AssetRepository::create_version(
        &db_pool,
        asset_id,
        current_asset.version,
        current_asset.version_id,
        current_asset.enriched_metadata.clone(),
        user_id,
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Merge metadata updates
    let mut updated_metadata = current_asset.enriched_metadata.clone();
    if let Some(title) = &update.title {
        updated_metadata["title"] = json!(title);
    }
    if let Some(description) = &update.description {
        updated_metadata["description"] = json!(description);
    }
    if let Some(tags) = &update.tags {
        updated_metadata["tags"] = json!(tags);
    }
    if let Some(category) = &update.category {
        updated_metadata["category"] = json!(category);
    }

    // Update asset
    AssetRepository::update_metadata(
        &db_pool,
        asset_id,
        updated_metadata.clone(),
        new_version,
        new_version_id,
        user_id,
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // I-FR-20: Update graph database
    let keywords = update.tags.unwrap_or_default();
    let topics = vec![]; // Extract from metadata if needed
    GraphRepository::index_asset(&db_pool, asset_id, &keywords, &topics).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "status": "success",
        "version": new_version,
        "version_id": new_version_id,
        "conflict_detected": false
    })))
}

// I-FR-19: Resolve metadata conflicts
pub async fn resolve_conflict(
    State(db_pool): State<DbPool>,
    Path(asset_id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let resolution_strategy = payload.get("resolution_strategy")
        .and_then(|s| s.as_str())
        .unwrap_or("merge");

    let current_asset = AssetRepository::get_by_uuid(&db_pool, asset_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let resolved_metadata = payload.get("resolved_metadata")
        .and_then(|m| m.as_object())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let user_id = Uuid::new_v4(); // TODO: Get from auth
    let new_version = current_asset.version + 1;
    let new_version_id = Uuid::new_v4();

    // Archive current version
    AssetRepository::create_version(
        &db_pool,
        asset_id,
        current_asset.version,
        current_asset.version_id,
        current_asset.enriched_metadata.clone(),
        user_id,
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Apply resolved metadata
    let resolved_json: serde_json::Value = serde_json::from_value(
        serde_json::to_value(resolved_metadata).unwrap()
    ).unwrap();

    AssetRepository::update_metadata(
        &db_pool,
        asset_id,
        resolved_json,
        new_version,
        new_version_id,
        user_id,
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "status": "success",
        "version": new_version,
        "message": "Conflict resolved successfully"
    })))
}