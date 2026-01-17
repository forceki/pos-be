use actix_web::{web, Responder, post, get};
use crate::dtos::auth_dto::{RegisterUserDTO,LoginUserDTO};
use crate::utils::api_response::ApiResponse;
use crate::app_state::AppState;

#[post("/register")]
pub async fn register(
    body: web::Json<RegisterUserDTO>,
    state: web::Data<AppState>,
) -> impl Responder {
    
    match state.auth_service.register_user(body.into_inner()).await {
        Ok(created_user) => {
            ApiResponse::response(created_user, Some("Register Succes".to_string()), actix_web::http::StatusCode::CREATED)
        },
        Err(e) => {
            ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[post("/login")]
pub async fn login(
    body: web::Json<LoginUserDTO>,
    state: web::Data<AppState>,
) -> impl Responder {
    match state.auth_service.login_user(body.into_inner()).await {
        Ok(token) => {
            ApiResponse::response(token, Some("Login successful".to_string()), actix_web::http::StatusCode::OK)
        },
        Err(e) => {
            ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::UNAUTHORIZED)
        }
    }
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    ApiResponse::response("OK", Some("Service is healthy".to_string()), actix_web::http::StatusCode::OK)
}

