use actix_web::HttpResponse;
use serde::Serialize;
use actix_web::http::StatusCode;
use crate::utils::pagination::PaginationMeta;


#[derive(Serialize)]
pub struct ResponseBody<T> {
    pub status: u16,
    pub message: Option<String>,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub struct ApiResponse;

impl ApiResponse {
    pub fn response<T: Serialize>(data: T, message: Option<String>, status_code: StatusCode) -> HttpResponse {
        HttpResponse::build(status_code).json(ResponseBody {
            status: status_code.as_u16(),
            message,
            data: Some(data),
            meta: None,
        })
    }

    pub fn response_paged<T: Serialize>(data: T, meta: Option<PaginationMeta>, message: Option<String>, status_code: StatusCode) -> HttpResponse {
        HttpResponse::build(status_code).json(ResponseBody::<T> {
            status: status_code.as_u16(),
            message,
            data: Some(data),
            meta,
        })
    }

    pub fn error(message: Option<String>, status_code: StatusCode) -> HttpResponse {
        HttpResponse::build(status_code).json(ResponseBody::<()> {
            status: status_code.as_u16(),
            message,
            data: None,
            meta: None,
        })
    }
}