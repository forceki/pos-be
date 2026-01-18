use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub link : Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}