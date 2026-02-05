use sqlx::{MySql, MySqlPool, QueryBuilder, FromRow};
use crate::models::brand_model::Brand;
use crate::dtos::brand_dto::{CreateBrandDto, UpdateBrandDto};

pub struct BrandRepository {
    pool: MySqlPool,
    company_id: String,
}

#[derive(FromRow)]
struct BrandWithCount {
    #[sqlx(flatten)]
    brand: Brand,
    total_count: i64,
}

impl BrandRepository {
    pub fn new(pool: MySqlPool, company_id: String) -> Self {
        Self { pool, company_id }
    }

    pub async fn create(&self, name: &str, slug: &str, user_id: &str) -> Result<u64, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO brands (name, slug, company_id, created_by) 
               VALUES (?, ?, ?, ?)"#,
            name, slug, self.company_id, user_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(res.last_insert_id())
    }

    pub async fn get_all(&self, limit: i64, offset: i64, search: Option<String>) -> Result<(Vec<Brand>, i64), sqlx::Error> {
        let mut qb: QueryBuilder<MySql> = QueryBuilder::new(
            "SELECT *, COUNT(*) OVER() as total_count FROM brands WHERE company_id = "
        );
        qb.push_bind(&self.company_id);

        if let Some(s) = search {
            qb.push(" AND name LIKE ");
            qb.push_bind(format!("%{}%", s));
        }

        qb.push(" ORDER BY created_at DESC LIMIT ");
        qb.push_bind(limit);
        qb.push(" OFFSET ");
        qb.push_bind(offset);

        let rows = qb.build_query_as::<BrandWithCount>()
            .fetch_all(&self.pool)
            .await?;

        let total = rows.first().map(|r| r.total_count).unwrap_or(0);
        let data = rows.into_iter().map(|r| r.brand).collect();

        Ok((data, total))
    }

    pub async fn update(&self, id: u64, dto: UpdateBrandDto, user_id: &str) -> Result<u64, sqlx::Error> {
        sqlx::query!(
            r#"UPDATE brands SET 
               name = COALESCE(?, name), 
               is_active = COALESCE(?, is_active),
               updated_by = ?,
               updated_at = NOW()
               WHERE id = ? AND company_id = ?"#,
            dto.name, dto.is_active, user_id, id, self.company_id
        )
        .execute(&self.pool)
        .await
        .map(|r| r.rows_affected())
    }
}