use sqlx::MySqlPool;

use crate::models::roles_model::Roles;


#[derive(Clone)]
pub struct RolesRepository {
    pub pool: MySqlPool
}

impl RolesRepository{
    pub fn new(pool: MySqlPool) -> Self {
        RolesRepository { pool}
    }

    pub async fn create(&self, role: &Roles) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO roles (id, name, tenant_id, description, created_at, updated_at) VALUES (?,?,?,?,?,?)",
            role.id,
            role.name,
            role.tenant_id,
            role.description,
            role.created_at,
            role.updated_at
        )
        .execute(&self.pool)
        .await?;


        Ok(())
    }

    pub async fn index(&self, limit: i64, offset: i64) -> Result<(Vec<Roles>, i64), sqlx::Error> {
        let count_result = sqlx::query!("SELECT count(1) as count FROM roles")
            .fetch_one(&self.pool)
            .await?;

        let total_items = count_result.count as i64;

        let roles = sqlx::query_as!(
            Roles,
            "SELECT id, name, description, tenant_id, created_at, updated_at FROM roles LIMIT ? OFFSET ?",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok((roles, total_items))
    }

}