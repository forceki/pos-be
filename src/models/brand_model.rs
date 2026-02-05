use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Brand {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub company_id: String,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}