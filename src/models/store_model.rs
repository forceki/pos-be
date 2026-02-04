use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Store {
    pub id: String,
    pub company_id: String,
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub store_number_phone: Option<String>, 
    pub slug: String,
    pub is_warehouse: i8,
    pub status: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}