use chrono::Utc;
use sqlx::MySqlPool;

use crate::{dtos::auth_dto::CreateOnboardDTO, models::user_model::User};



#[derive(Clone)]
pub struct AuthRepository {
    pool: MySqlPool
}

impl  AuthRepository {
    pub fn new(pool: MySqlPool) -> Self {
        AuthRepository { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, fullname, email, password, role_id, tenant_id, created_at, updated_at FROM users WHERE email = ?",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn onboard(&self, payload: CreateOnboardDTO) -> Result<String, sqlx::Error>{

        let mut tx = self.pool.begin().await?;
        
        sqlx::query!(
            "INSERT INTO company (id, name, slug, created_at) VALUES (?, ?, ?, ?)",
            payload.company_id, payload.company_slug, payload.company_name, Utc::now()
        )
        .execute(&mut *tx).await?;

        sqlx::query!(
            "INSERT INTO roles (id, name, description, tenant_id, created_at) VALUES (?, ?, ?, ?, ?)",
            payload.role_id, payload.role_name, "Pemilik Toko", payload.company_id, Utc::now()
        )
        .execute(&mut *tx).await?;

        sqlx::query!(
            r#"
            INSERT INTO users (user_id, fullname, email, password, role_id, tenant_id, created_at, updated_at) 
            VALUES (?, ?, ?, ?, ?, ?, ?,?)
            "#,
            payload.user_id,
            payload.fullname,
            payload.email,
            payload.password,
            payload.role_id,
            payload.company_id,
            Utc::now(),
            Utc::now()
        )
        .execute(&mut *tx).await?;


        tx.commit().await?;

        Ok(payload.user_id)
    }
}