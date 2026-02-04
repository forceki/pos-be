use serde::{Deserialize, Serialize};

use crate::{dtos::pagination_dto::PaginationQuery, models::category_model::Category};

#[derive(Deserialize)]
pub struct CreateCategoryDto {
    pub name: String,
    pub parent_id: Option<u64>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateCategoryDto {
    pub name: Option<String>,
    pub parent_id: Option<u64>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize)]
pub struct CategoryQuery {
    pub search: Option<String>,

    #[serde(flatten)]
    pub pagination: PaginationQuery, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryResponse {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub parent_id: Option<u64>,
    pub is_active: bool,
    pub children: Vec<CategoryResponse>,
}

impl From<Category> for CategoryResponse {
    fn from(item: Category) -> Self {
        CategoryResponse {
            id: item.id,
            name: item.name,
            slug: item.slug,
            parent_id: item.parent_id,
            is_active: item.is_active,
            children: Vec::new()
        }
    }
}