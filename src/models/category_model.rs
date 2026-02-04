use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Category {
    pub id: u64, // BIGINT UNSIGNED di MySQL -> u64 di Rust
    pub name: String,
    pub slug: String,
    pub company_id: String,
    pub parent_id: Option<u64>,
    pub is_active: bool, // TINYINT(1) otomatis jadi bool oleh sqlx
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}