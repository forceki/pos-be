use actix_web::{HttpMessage, HttpRequest, Responder, get, post, web};

use crate::{app_state::AppState, dtos::{pagination_dto::PaginationQuery, roles_dto::CreateRolesDTO}, utils::{api_response::ApiResponse, token_utils::Claims}};



#[post("/")]
pub async fn create(
    body: web::Json<CreateRolesDTO>,
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl  Responder {
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.roles_service(claims.company_id);

    match service.create(body.into_inner()).await  {
        Ok(role) => {
            ApiResponse::response(role, Some("Succes".to_string()), actix_web::http::StatusCode::CREATED)
        }
        Err(e) => {
            ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


#[get["/"]]
pub async  fn index(
    state: web::Data<AppState>,
    query : web::Query<PaginationQuery>,
    req: HttpRequest
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.roles_service(claims.company_id);

    match  service.get_all(query.into_inner()).await {
        Ok((data, meta)) => {
            ApiResponse::response_paged(data, Some(meta), Some("Users retrieved successfully".to_string()), actix_web::http::StatusCode::OK)
        },
        Err(e) => {
            ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}