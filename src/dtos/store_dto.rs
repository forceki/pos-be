use serde::Deserialize;

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