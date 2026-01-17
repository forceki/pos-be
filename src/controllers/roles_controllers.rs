use actix_web::{Responder, post, web};

use crate::{app_state::AppState, dtos::roles_dto::CreateRolesDTO, utils::api_response::ApiResponse};



#[post("/")]
pub async fn create(
    body: web::Json<CreateRolesDTO>,
    state: web::Data<AppState>
) -> impl  Responder {
    match state.role_service.create(body.into_inner()).await  {
        Ok(role) => {
            ApiResponse::response(role, Some("Succes".to_string()), actix_web::http::StatusCode::CREATED)
        }
        Err(e) => {
            ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}