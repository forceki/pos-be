use sqlx::MySqlPool;
use crate::models::user_model::User;

#[derive(Clone)]
pub struct UserRepository {
    pub pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        UserRepository { pool }
    }


    pub async fn create(&self, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (user_id, username, fullname, email, password, role_id, tenant_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            user.user_id,
            user.username,
            user.fullname,
            user.email,
            user.password,
            user.role_id,
            user.tenant_id,
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await?;

        let created = sqlx::query_as!(
            User,
            "SELECT user_id, username, fullname, email, password, role_id, tenant_id, created_at, updated_at FROM users WHERE user_id = ?",
            user.user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, username, fullname, email, password, role_id, tenant_id, created_at, updated_at FROM users WHERE email = ?",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, username, fullname, email, password, role_id, tenant_id, created_at, updated_at FROM users WHERE user_id = ?",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn fetch_all(&self, limit: i64, offset: i64) -> Result<(Vec<User>, i64), sqlx::Error> {

        let count_result = sqlx::query!("SELECT count(*) as count FROM users")
            .fetch_one(&self.pool)
            .await?;

        let total_items = count_result.count as i64;

        let users = sqlx::query_as!(
            User,
            "SELECT user_id, username, fullname, email, password, role_id, tenant_id, created_at, updated_at FROM users LIMIT ? OFFSET ?",
            limit, 
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok((users, total_items))
    }
}