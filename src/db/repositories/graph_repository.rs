// Graph repository
// I-FR-20: Graph indexing
// I-FR-22: Graph search

use crate::db::DbPool;
use anyhow::Result;
use uuid::Uuid;

pub struct GraphRepository;

impl GraphRepository {
    // I-FR-20: Index asset in graph
    pub async fn index_asset(
        pool: &DbPool,
        asset_uuid: Uuid,
        keywords: &[String],
        topics: &[String],
    ) -> Result<()> {
        // Create keyword nodes and relationships
        for keyword in keywords {
            // Get or create keyword node
            let node_id: Uuid = sqlx::query_scalar(
                r#"
                INSERT INTO graph_nodes (node_id, node_type, node_name, created_at)
                VALUES (uuid_generate_v4(), 'KEYWORD', $1, NOW())
                ON CONFLICT (node_type, node_name) DO UPDATE SET node_name = $1
                RETURNING node_id
                "#
            )
            .bind(keyword)
            .fetch_one(pool.as_ref())
            .await?;

            // Link asset to keyword
            sqlx::query(
                r#"
                INSERT INTO asset_graph_nodes (asset_uuid, node_id, created_at)
                VALUES ($1, $2, NOW())
                ON CONFLICT DO NOTHING
                "#
            )
            .bind(asset_uuid)
            .bind(node_id)
            .execute(pool.as_ref())
            .await?;
        }

        // Create topic nodes and relationships
        for topic in topics {
            let node_id: Uuid = sqlx::query_scalar(
                r#"
                INSERT INTO graph_nodes (node_id, node_type, node_name, created_at)
                VALUES (uuid_generate_v4(), 'TOPIC', $1, NOW())
                ON CONFLICT (node_type, node_name) DO UPDATE SET node_name = $1
                RETURNING node_id
                "#
            )
            .bind(topic)
            .fetch_one(pool.as_ref())
            .await?;

            sqlx::query(
                r#"
                INSERT INTO asset_graph_nodes (asset_uuid, node_id, created_at)
                VALUES ($1, $2, NOW())
                ON CONFLICT DO NOTHING
                "#
            )
            .bind(asset_uuid)
            .bind(node_id)
            .execute(pool.as_ref())
            .await?;
        }

        Ok(())
    }

    // I-FR-22: Search graph
    pub async fn search(
        pool: &DbPool,
        query: &str,
        asset_type: Option<&str>,
        limit: i64,
    ) -> Result<Vec<Uuid>> {
        let keywords: Vec<&str> = query.split_whitespace().collect();
        
        let assets = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT DISTINCT a.uuid
            FROM assets a
            INNER JOIN asset_graph_nodes agn ON a.uuid = agn.asset_uuid
            INNER JOIN graph_nodes gn ON agn.node_id = gn.node_id
            WHERE (
                gn.node_name ILIKE ANY($1) OR
                a.asset_name ILIKE ANY($1) OR
                a.enriched_metadata::text ILIKE ANY($1)
            )
            AND ($2::text IS NULL OR a.asset_type::text = $2)
            ORDER BY a.created_at DESC
            LIMIT $3
            "#
        )
        .bind(keywords.iter().map(|k| format!("%{}%", k)).collect::<Vec<_>>())
        .bind(asset_type)
        .bind(limit)
        .fetch_all(pool.as_ref())
        .await?;

        Ok(assets)
    }

    // Create relationship between assets
    pub async fn create_relationship(
        pool: &DbPool,
        source_uuid: Uuid,
        target_uuid: Uuid,
        relationship_type: &str,
        relationship_data: Option<serde_json::Value>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO graph_relationships (
                source_asset_uuid, target_asset_uuid, relationship_type, relationship_data, created_at
            ) VALUES ($1, $2, $3, $4, NOW())
            ON CONFLICT DO NOTHING
            "#
        )
        .bind(source_uuid)
        .bind(target_uuid)
        .bind(relationship_type)
        .bind(relationship_data)
        .execute(pool.as_ref())
        .await?;

        Ok(())
    }
}
