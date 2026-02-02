use actix_web::{HttpMessage, HttpRequest, Responder, post, web};

use crate::{app_state::AppState, dtos::company_dto::CreateCompanyDTO, utils::{api_response::ApiResponse, token_utils::Claims}};


#[post("/")]
pub async fn create(
    body: web::Json<CreateCompanyDTO>,
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.company_service(claims.tenant_id);
    
    match service.create(body.into_inner()).await {
        Ok(role) => {
            ApiResponse::response(role, Some("Succes".to_string()), actix_web::http::StatusCode::CREATED)
        }
        Err(e) => {
            ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}