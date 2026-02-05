use actix_web::{HttpMessage, HttpRequest, Responder, get, post, put, web};
use crate::app_state::AppState;
use crate::dtos::store_dto::{ArchiveStoreDto, CreateStoreDto, StoreQuery, UpdateStoreDto};
use crate::utils::{api_response::ApiResponse, token_utils::Claims};

#[post("/")]
pub async fn create_store(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateStoreDto>,
) -> impl Responder {
    
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    
    let service = state.store_service(claims.company_id);

    match service.create_store(body.into_inner()).await {
        Ok(id) => ApiResponse::response(id, Some("Store created successfully".to_string()), actix_web::http::StatusCode::CREATED),
        Err(e) => ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::BAD_REQUEST),
    }
}

#[get("/")]
pub async fn get_stores(
    state: web::Data<AppState>,
    query: web::Query<StoreQuery>,
    req: HttpRequest
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.store_service(claims.company_id);

    match service.get_all_stores(query.into_inner()).await {
        Ok((stores, meta)) =>ApiResponse::response_paged(stores, Some(meta), Some("Store retrieved successfully".to_string()), actix_web::http::StatusCode::OK),
        Err(e) => ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[post("/archive")]
pub async fn archive_store(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<ArchiveStoreDto>,
) -> impl Responder {

    let claims = req.extensions().get::<Claims>().unwrap().clone();
    
    let service = state.store_service(claims.company_id);

    match service.update_status_store(body.into_inner()).await {
        Ok(_) => ApiResponse::response((), Some("Store archived successfully".to_string()), actix_web::http::StatusCode::OK),
        Err(e) => ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::from_u16(e.as_response_error().status_code().as_u16()).unwrap()),
    }
}

#[put("/{id}")]
pub async fn update_store(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<String>, // Ambil ID dari URL
    body: web::Json<UpdateStoreDto>,
) -> impl Responder {
    let id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.store_service(claims.company_id);

    match service.update_store(&id, body.into_inner()).await {
        Ok(_) => ApiResponse::response((), Some("Store updated successfully".to_string()), actix_web::http::StatusCode::OK),
        Err(e) => ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::from_u16(e.as_response_error().status_code().as_u16()).unwrap()),
    }
}