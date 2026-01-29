use std::ptr::null;

use actix_web::{HttpMessage, HttpRequest, Responder, get, web};
use futures_util::io::empty;
use crate::models::user_model::UserPath;
use crate::utils::api_response::ApiResponse;
use crate::dtos::pagination_dto::PaginationQuery;
use crate::app_state::AppState;
use crate::utils::token_utils::Claims;

#[get("/")]
pub async fn get_users(
    state: web::Data<AppState>,
    query : web::Query<PaginationQuery>,
) -> impl Responder {
    match state.users_service.get_all_users(query.into_inner()).await {
        Ok((users, meta)) => {
            ApiResponse::response_paged(users, Some(meta), Some("Users retrieved successfully".to_string()), actix_web::http::StatusCode::OK)
        },
        Err(e) => {
            ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[get("/check")]
pub async fn check_users(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    
    if let Some(claims) = req.extensions().get::<Claims>() {
        println!("User ID: {}", claims.sub);
        println!("Tenant ID: {}", claims.tenant_id);
        println!("Role: {}", claims.role);
    } else {
        return ApiResponse::error(Some("Unauthorized".to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    return ApiResponse::response(Some(()), Some("success".to_string()), actix_web::http::StatusCode::OK)
}
#[get("/{id}")]
pub async fn get_users_by_id(
    state: web::Data<AppState>,
    path: web::Path<UserPath>
) -> impl Responder {

    let path = path.into_inner();

    match  state.users_service.user_by_id(&path.id).await {
        Ok(user) => {
            ApiResponse::response(user, Some("success".to_string()), actix_web::http::StatusCode::OK)
        },
        Err(e) => {
            ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}