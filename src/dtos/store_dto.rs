use serde::Deserialize;

use crate::{dtos::pagination_dto::PaginationQuery, models::store_model::Store};

#[derive(Deserialize)]
pub struct CreateStoreDto {
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub store_number_phone: Option<String>,
    pub is_warehouse: i8
}


#[derive(Deserialize)]
pub struct UpdateStoreDto {
    pub name: Option<String>,
    pub address: Option<String>,
    pub store_number_phone: Option<String>,
    pub is_warehouse: i8,
    pub status: Option<i8>,
}

#[derive(Deserialize)]
pub struct ArchiveStoreDto {
    pub id: String, 
    pub status : i8
}

#[derive(Deserialize)]
pub struct StoreQuery {
    pub search: Option<String>,
    pub status: Option<i8>,

    #[serde(flatten)]
    pub pagination: PaginationQuery, 
}