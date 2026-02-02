use sqlx::MySqlPool;

use crate::models::company_model::Company;


#[derive(Clone)]
pub struct CompanyRepository{
    pub pool: MySqlPool,
    pub tenant_id: String
}

impl CompanyRepository {
    pub fn new(pool: MySqlPool, tenant_id: String) -> Self {
        CompanyRepository { pool, tenant_id }
    }

    pub async fn create(&self, data: &Company) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO company (id, name, slug, address, phone_number, link,created_at, updated_at) VALUES (?,?,?,?,?,?,?,?)",
            data.id,
            data.name,
            data.slug,
            data.address,
            data.phone_number,
            data.link,
            data.created_at,
            data.updated_at,

        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}