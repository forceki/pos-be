use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PaginationQuery {
    pub page: Option<i64>, 
    pub limit: Option<i64>,
}

impl PaginationQuery {
    pub fn get_page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn get_limit(&self) -> i64 {
        self.limit.unwrap_or(10).max(1) 
    }

    pub fn get_offset(&self) -> i64 {
        (self.get_page() - 1) * self.get_limit()
    }
}