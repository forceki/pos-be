
use actix_web::{HttpMessage, HttpRequest, Responder, get, post, web};
use crate::dtos::auth_dto::RegisterUserDTO;
use crate::models::user_model::UserPath;
use crate::utils::api_response::ApiResponse;
use crate::dtos::pagination_dto::PaginationQuery;
use crate::app_state::AppState;
use crate::utils::token_utils::Claims;

#[get("/")]
pub async fn get_users(
    state: web::Data<AppState>,
    query : web::Query<PaginationQuery>,
    req: HttpRequest
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.users_service(claims.company_id);

    match service.get_all_users(query.into_inner()).await {
        Ok((users, meta)) => {
            ApiResponse::response_paged(users, Some(meta), Some("Users retrieved successfully".to_string()), actix_web::http::StatusCode::OK)
        },
        Err(e) => {
            ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[get("/check")]
pub async fn check_users(req: HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let claims_data = claims.clone(); 
        
        return ApiResponse::response(
            claims_data,
            Some("success".to_string()), 
            actix_web::http::StatusCode::OK
        );
    } 

    ApiResponse::error(
        Some("Unauthorized".to_string()), 
        actix_web::http::StatusCode::UNAUTHORIZED
    )
}

#[post("/register")]
pub async fn register(
    body: web::Json<RegisterUserDTO>,
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.users_service(claims.company_id);
    match service.register_user(body.into_inner()).await {
        Ok(created_user) => {
            ApiResponse::response(created_user, Some("Register Succes".to_string()), actix_web::http::StatusCode::CREATED)
        },
        Err(e) => {
            ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[get("/{id}")]
pub async fn get_users_by_id(
    state: web::Data<AppState>,
    path: web::Path<UserPath>,
    req: HttpRequest
) -> impl Responder {

    let path = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.users_service(claims.company_id);

    match  service.user_by_id(&path.id).await {
        Ok(user) => {
            ApiResponse::response(user, Some("success".to_string()), actix_web::http::StatusCode::OK)
        },
        Err(e) => {
            ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}