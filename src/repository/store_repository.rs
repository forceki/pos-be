use sqlx::{MySql, MySqlPool, QueryBuilder, prelude::FromRow};
use crate::{dtos::store_dto::UpdateStoreDto, models::{self, store_model::{Store}}};

#[derive(Clone)]
pub struct StoreRepository {
    pool: MySqlPool,
    company_id: String,
}


#[derive(FromRow)]
struct StoreWithCount {
    #[sqlx(flatten)]
    store: Store,
    total_count: i64,
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


    pub async fn get_all(
        &self,
        limit: i64,
        offset: i64,
        status: Option<i8>, 
        search: Option<String>,
    ) -> Result<(Vec<Store>, i64), sqlx::Error> {

        let mut qb: QueryBuilder<MySql> = QueryBuilder::new(
            "SELECT *, COUNT(*) OVER() as total_count FROM stores WHERE company_id = "
        );
        qb.push_bind(&self.company_id);

        if let Some(keyword) = &search {
            qb.push(" AND (name LIKE ");
            qb.push_bind(format!("%{}%", keyword));
            qb.push(" OR code LIKE ");
            qb.push_bind(format!("%{}%", keyword));
            qb.push(")");
        }

        if let Some(s) = status {
            qb.push(" AND status = "); 
            qb.push_bind(s);
        }

        qb.push(" ORDER BY created_at DESC LIMIT ");
        qb.push_bind(limit);
        qb.push(" OFFSET ");
        qb.push_bind(offset);

        let rows = qb.build_query_as::<StoreWithCount>()
            .fetch_all(&self.pool)
            .await?;

        let total = rows.first().map(|r| r.total_count).unwrap_or(0);
        
        // Ambil store-nya saja (consume vector rows biar hemat memori)
        let data = rows.into_iter().map(|r| r.store).collect();

        Ok((data, total))
    }

    pub async  fn update(&self, id: &str, dto: &UpdateStoreDto) -> Result<u64, sqlx::Error> {
        let is_warehouse_db = dto.is_warehouse;

        sqlx::query!(
            r#"
                UPDATE stores 
                SET 
                    name = COALESCE(?, name),
                    address = COALESCE(?, address),
                    store_number_phone = COALESCE(?, store_number_phone),
                    is_warehouse = COALESCE(?, is_warehouse),
                    status = COALESCE(?, status),
                    updated_at = NOW()
                WHERE id = ? AND company_id = ?
            "#,
            dto.name,
            dto.address,
            dto.store_number_phone,
            is_warehouse_db,
            dto.status,
            id,
            self.company_id
        ).
        execute(&self.pool)
        .await
        .map(|result| result.rows_affected())

    }

    pub async fn archive_unarchive(&self, id: &str, status: i8) -> Result<u64, sqlx::Error> {
        sqlx::query!(
            "UPDATE stores SET status = ? , updated_at = NOW() WHERE id = ? AND company_id = ?",
            status,
            id,
            self.company_id
        )
        .execute(&self.pool)
        .await
        .map(|result| result.rows_affected())
    }
    
    pub async fn exists(&self, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT count(1) as count FROM stores WHERE id = ? AND company_id = ?",
            id,
            self.company_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.count > 0)
    }
}