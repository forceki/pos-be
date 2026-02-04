use chrono::Utc;
use sqlx::{MySql, MySqlPool, QueryBuilder, FromRow};
use crate::models::category_model::Category;
use crate::dtos::category_dto::{CreateCategoryDto, UpdateCategoryDto};

pub struct CategoryRepository {
    pool: MySqlPool,
    company_id: String,
}

#[derive(FromRow)]
struct CategoryWithCount {
    #[sqlx(flatten)]
    category: Category,
    total_count: i64,
}

impl CategoryRepository {
    pub fn new(pool: MySqlPool, company_id: String) -> Self {
        Self { pool, company_id }
    }

    pub async fn create(&self, name: &str, slug: &str, parent_id: Option<u64>, user_id: &str) -> Result<u64, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO categories (name, slug, company_id, parent_id, created_by, created_at) 
               VALUES (?, ?, ?, ?, ?, ?)"#,
            name, slug, self.company_id, parent_id, user_id, Utc::now()
        )
        .execute(&self.pool)
        .await?;
        
        Ok(res.last_insert_id())
    }

    pub async fn get_all(&self, limit: i64, offset: i64, search: Option<String>) -> Result<(Vec<Category>, i64), sqlx::Error> {
        let mut qb: QueryBuilder<MySql> = QueryBuilder::new(
            "SELECT *, COUNT(*) OVER() as total_count FROM categories WHERE company_id = "
        );
        qb.push_bind(&self.company_id);

        if let Some(s) = search {
            qb.push(" AND name LIKE ");
            qb.push_bind(format!("%{}%", s));
        }

        qb.push(" ORDER BY name ASC LIMIT ");
        qb.push_bind(limit);
        qb.push(" OFFSET ");
        qb.push_bind(offset);

        let rows = qb.build_query_as::<CategoryWithCount>()
            .fetch_all(&self.pool)
            .await?;

        let total = rows.first().map(|r| r.total_count).unwrap_or(0);
        let data = rows.into_iter().map(|r| r.category).collect();

        Ok((data, total))
    }

    pub async fn update(&self, id: u64, dto: UpdateCategoryDto, user_id: &str) -> Result<u64, sqlx::Error> {
        sqlx::query!(
            r#"UPDATE categories SET 
               name = COALESCE(?, name), 
               parent_id = COALESCE(?, parent_id),
               is_active = COALESCE(?, is_active),
               updated_by = ?,
               updated_at = ?
               WHERE id = ? AND company_id = ?"#,
            dto.name, dto.parent_id, dto.is_active, user_id, id, self.company_id, Utc::now()
        )
        .execute(&self.pool)
        .await
        .map(|r| r.rows_affected())
    }
}