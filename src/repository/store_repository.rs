use sqlx::MySqlPool;
use crate::models::store_model::Store;

#[derive(Clone)]
pub struct StoreRepository {
    pool: MySqlPool,
    company_id: String,
}

impl StoreRepository {
    pub fn new(pool: MySqlPool, company_id: String) -> Self {
        Self { pool, company_id }
    }

    pub async fn exists_by_code(&self, code: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT count(1) as count FROM stores WHERE company_id = ? AND code = ?",
            self.company_id,
            code
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.count > 0)
    }

    pub async fn create(&self, new_store: &Store) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO stores (
                id, company_id, code, name, address, store_number_phone, 
                slug, is_warehouse, status, created_at, updated_at
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_store.id,
            self.company_id,
            new_store.code,
            new_store.name,
            new_store.address,
            new_store.store_number_phone,
            new_store.slug,
            new_store.is_warehouse,
            new_store.status,
            new_store.created_at,
            new_store.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all(&self, limit: i64, offset: i64) -> Result<(Vec<Store>, i64), sqlx::Error> {
        let count =  sqlx::query!("SELECT count(1) as count FROM stores WHERE company_id = ?",self.company_id)
            .fetch_one(&self.pool)
            .await?;

        let total_items = count.count as i64;


        let data = sqlx::query_as!(
            Store,
            r#"
                SELECT * FROM stores 
                WHERE company_id = ? 
                ORDER BY created_at DESC 
                LIMIT ? OFFSET ?
            "#,
            self.company_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;


        Ok((data, total_items))
    }
}