use serde::Deserialize;

use crate::dtos::pagination_dto::PaginationQuery;

#[derive(Deserialize)]
pub struct CreateBrandDto {
    pub name: String,
    pub is_active: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateBrandDto {
    pub name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize)]
pub struct BrandQuery {
    pub search: Option<String>,

    #[serde(flatten)]
    pub pagination: PaginationQuery, 
}