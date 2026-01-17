use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub role_id: String,
    pub tenant_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize,Serialize)]
pub struct UserPath {
   pub id: String,
}