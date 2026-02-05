use actix_web::{HttpMessage, HttpRequest, Responder, get, post, put, web};
use crate::app_state::AppState;
use crate::dtos::brand_dto::{BrandQuery, CreateBrandDto, UpdateBrandDto};
use crate::utils::api_response::ApiResponse;
use crate::utils::token_utils::Claims;

#[post("/")]
pub async fn create_brand(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateBrandDto>,
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.brand_service(claims.company_id);

    match service.create_brand(body.into_inner(), &claims.userid).await {
        Ok(id) => ApiResponse::response(id, Some("Brand created".to_string()), actix_web::http::StatusCode::CREATED),
        Err(e) => ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::BAD_REQUEST),
    }
}

#[get("/")]
pub async fn get_brands(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<BrandQuery>,
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.brand_service(claims.company_id);
    
    let params = query.into_inner();
    match service.get_brands(params.pagination.get_limit(), params.pagination.get_offset(), params.search, params.pagination.get_page()).await {
        Ok((brands, meta)) => ApiResponse::response_paged(brands, Some(meta), Some("Brands retrieved".to_string()), actix_web::http::StatusCode::OK),
        Err(e) => ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[put("/{id}")]
pub async fn update_brand(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<u64>,
    body: web::Json<UpdateBrandDto>,
) -> impl Responder {
    let id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.brand_service(claims.company_id);

    match service.update_brand(id, body.into_inner(), &claims.userid).await {
        Ok(_) => ApiResponse::response((), Some("Brand updated".to_string()), actix_web::http::StatusCode::OK),
        Err(e) => ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::NOT_FOUND),
    }
}