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
            "INSERT INTO roles (id, name, description, created_at, updated_at) VALUES (?,?,?,?,?)",
            role.id,
            role.name,
            role.description,
            role.created_at,
            role.updated_at
        )
        .execute(&self.pool)
        .await?;


        Ok(())
    }

}