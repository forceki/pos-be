use chrono::Utc;
use sqlx::MySqlPool;
use crate::{dtos::auth_dto::{CreateOnboardDTO}, models::user_model::User};

#[derive(Clone)]
pub struct UserRepository {
    pool: MySqlPool, 
    company_id: String, 
}
impl UserRepository {
    pub fn new(pool: MySqlPool, company_id: String) -> Self {
        UserRepository { pool, company_id }
    }

    pub async fn create(&self, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (user_id, fullname, email, password, role_id, company_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            user.user_id,
            user.fullname,
            user.email,
            user.password,
            user.role_id,
            self.company_id,
            user.created_at,
            user.updated_at            
        )
        .execute(&self.pool)
        .await?;

        let created = sqlx::query_as!(
            User,
            "SELECT user_id, fullname, email, password, role_id, company_id, created_at, updated_at FROM users WHERE user_id = ?",
            user.user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, fullname, email, password, role_id, company_id, created_at, updated_at FROM users WHERE email = ?",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, fullname, email, password, role_id, company_id, created_at, updated_at FROM users WHERE user_id = ?",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn fetch_all(&self, limit: i64, offset: i64) -> Result<(Vec<User>, i64), sqlx::Error> {

        let count_result = sqlx::query!("SELECT count(1) as count FROM users WHERE company_id = ?",self.company_id)
            .fetch_one(&self.pool)
            .await?;

        let total_items = count_result.count as i64;

        let users = sqlx::query_as!(
            User,
            "SELECT user_id, fullname, email, password, role_id, company_id, created_at, updated_at FROM users WHERE company_id = ? LIMIT ? OFFSET ?",
            self.company_id,
            limit, 
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok((users, total_items))
    }
}