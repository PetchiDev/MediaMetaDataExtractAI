// User repository
// I-FR-21: SSO integration
// I-FR-23: API key management

use crate::db::DbPool;
use crate::models::user::{ApiKey, User};
use anyhow::Result;
use sqlx::Row;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn get_by_email(pool: &DbPool, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(pool.as_ref())
        .await?;

        Ok(user)
    }

    pub async fn get_by_sso_id(pool: &DbPool, sso_provider_id: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE sso_provider_id = $1"
        )
        .bind(sso_provider_id)
        .fetch_optional(pool.as_ref())
        .await?;

        Ok(user)
    }

    pub async fn create(pool: &DbPool, user: &User) -> Result<Uuid> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO users (id, email, name, role, sso_provider_id, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(&user.role)
        .bind(&user.sso_provider_id)
        .bind(user.created_at)
        .fetch_one(pool.as_ref())
        .await?;

        Ok(id)
    }

    pub async fn get_by_id(pool: &DbPool, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool.as_ref())
        .await?;

        Ok(user)
    }

    // API Key operations
    pub async fn create_api_key(pool: &DbPool, api_key: &ApiKey) -> Result<Uuid> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO api_keys (
                id, user_id, key_name, key_hash, permissions, created_at, status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(&api_key.id)
        .bind(&api_key.user_id)
        .bind(&api_key.key_name)
        .bind(&api_key.key_hash)
        .bind(serde_json::to_value(&api_key.permissions)?)
        .bind(api_key.created_at)
        .bind(&api_key.status)
        .fetch_one(pool.as_ref())
        .await?;

        Ok(id)
    }

    pub async fn get_api_key_by_hash(pool: &DbPool, key_hash: &str) -> Result<Option<ApiKey>> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, key_name, key_hash, permissions, created_at, last_used, status
            FROM api_keys WHERE key_hash = $1 AND status = 'ACTIVE'
            "#
        )
        .bind(key_hash)
        .fetch_optional(pool.as_ref())
        .await?;

        if let Some(row) = row {
            Ok(Some(ApiKey {
                id: row.get("id"),
                user_id: row.get("user_id"),
                key_name: row.get("key_name"),
                key_hash: row.get("key_hash"),
                permissions: serde_json::from_value(row.get("permissions"))?,
                created_at: row.get("created_at"),
                last_used: row.get("last_used"),
                status: row.get("status"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_api_keys(pool: &DbPool, user_id: Uuid) -> Result<Vec<ApiKey>> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, key_name, key_hash, permissions, created_at, last_used, status
            FROM api_keys WHERE user_id = $1 ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(pool.as_ref())
        .await?;

        let mut keys = Vec::new();
        for row in rows {
            keys.push(ApiKey {
                id: row.get("id"),
                user_id: row.get("user_id"),
                key_name: row.get("key_name"),
                key_hash: row.get("key_hash"),
                permissions: serde_json::from_value(row.get("permissions"))?,
                created_at: row.get("created_at"),
                last_used: row.get("last_used"),
                status: row.get("status"),
            });
        }

        Ok(keys)
    }

    pub async fn revoke_api_key(pool: &DbPool, key_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE api_keys SET status = 'REVOKED' WHERE id = $1"
        )
        .bind(key_id)
        .execute(pool.as_ref())
        .await?;

        Ok(())
    }
}
