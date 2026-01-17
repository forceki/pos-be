use serde::Serialize;

#[derive(Serialize)]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Serialize, Clone)]
pub struct PaginationMeta {
    pub page: i64,
    pub limit: i64,
    pub total_items: i64,
    pub total_pages: i64,
}

impl PaginationMeta {
    pub fn new(page: i64, limit: i64, total_items: i64) -> Self {
        let total_pages = if limit == 0 {
            1 
        } else {
            (total_items as f64 / limit as f64).ceil() as i64
        };

        Self {
            page,
            limit,
            total_items,
            total_pages,
        }
    }
}