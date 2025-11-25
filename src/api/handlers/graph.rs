// Graph search handlers - FULL IMPLEMENTATION
// I-FR-20: Graph indexing
// I-FR-22: Graph-based search

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use crate::db::DbPool;
use crate::db::repositories::{graph_repository::GraphRepository, asset_repository::AssetRepository};
use serde_json::json;
use sqlx::Row;

// I-FR-22: Graph-based content discovery
pub async fn search(
    State(db_pool): State<DbPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let query = payload.get("query")
        .and_then(|q| q.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let filters = payload.get("filters").and_then(|f| f.as_object());
    let asset_type = filters
        .and_then(|f| f.get("asset_type"))
        .and_then(|a| a.as_str());

    let limit = payload.get("max_results")
        .and_then(|l| l.as_i64())
        .unwrap_or(50);

    // Search graph database
    let asset_uuids = GraphRepository::search(&db_pool, query, asset_type, limit).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get full asset details
    let mut assets = Vec::new();
    for uuid in asset_uuids {
        if let Ok(Some(asset)) = AssetRepository::get_by_uuid(&db_pool, uuid).await {
            assets.push(json!({
                "uuid": asset.uuid,
                "name": asset.asset_name,
                "type": format!("{:?}", asset.asset_type),
                "title": asset.enriched_metadata.get("title"),
                "description": asset.enriched_metadata.get("description"),
                "created_at": asset.created_at,
            }));
        }
    }

    Ok(Json(json!({
        "assets": assets,
        "total_results": assets.len(),
        "query": query
    })))
}

pub async fn get_relationships(
    State(db_pool): State<DbPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let asset_uuid = payload.get("asset_uuid")
        .and_then(|u| u.as_str())
        .and_then(|s| uuid::Uuid::parse_str(s).ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    // Get relationships from graph
    let relationships = sqlx::query(
        r#"
        SELECT target_asset_uuid, relationship_type, relationship_data
        FROM graph_relationships
        WHERE source_asset_uuid = $1
        "#
    )
    .bind(asset_uuid)
    .fetch_all(db_pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: Vec<serde_json::Value> = relationships.iter().map(|row: &sqlx::postgres::PgRow| {
        json!({
            "target_asset_uuid": row.get::<uuid::Uuid, _>("target_asset_uuid"),
            "relationship_type": row.get::<String, _>("relationship_type"),
            "relationship_data": row.get::<Option<serde_json::Value>, _>("relationship_data"),
        })
    }).collect();

    Ok(Json(json!({
        "source_asset_uuid": asset_uuid,
        "relationships": result
    })))
}