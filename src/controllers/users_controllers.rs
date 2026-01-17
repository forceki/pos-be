use actix_web::{web, Responder, get};
use crate::models::user_model::UserPath;
use crate::utils::api_response::ApiResponse;
use crate::dtos::pagination_dto::PaginationQuery;
use crate::app_state::AppState;


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