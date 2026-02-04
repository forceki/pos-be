use actix_web::{HttpMessage, HttpRequest, Responder, get, post, put, web};
use crate::app_state::AppState;
use crate::dtos::category_dto::{CategoryQuery, CreateCategoryDto, UpdateCategoryDto};
use crate::utils::{api_response::ApiResponse, token_utils::Claims};



#[get("/")]
pub async fn get_all(
    state: web::Data<AppState>,
    query: web::Query<CategoryQuery>,
    req: HttpRequest
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.category_service(claims.company_id);

    
    match service.get_all(query.into_inner()).await {
        Ok((stores, meta)) =>ApiResponse::response_paged(stores, Some(meta), Some("Category retrieved successfully".to_string()), actix_web::http::StatusCode::OK),
        Err(e) => ApiResponse::error(Some(e.to_string()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[get("/tree")]
pub async fn get_category_tree(
    req: HttpRequest,
    query: web::Query<CategoryQuery>,
    state: web::Data<AppState>,
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.category_service(claims.company_id);

    match service.get_tree(query.into_inner()).await {
        Ok(tree) => ApiResponse::response(tree, Some("Succes".to_string()), actix_web::http::StatusCode::OK),
        Err(e) => ApiResponse::error(
            Some(e.to_string()), 
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        ),
    }
}

#[post("/")]
pub async fn create_category(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateCategoryDto>,
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.category_service(claims.company_id);

    match service.create(body.into_inner(), &claims.userid).await {
        Ok(id) => ApiResponse::response(
            id, 
            Some("Category created successfully".to_string()), 
            actix_web::http::StatusCode::CREATED
        ),
        Err(e) => ApiResponse::error(
            Some(e.to_string()), 
            actix_web::http::StatusCode::BAD_REQUEST
        ),
    }
}

#[put("/{id}")]
pub async fn update_category(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<u64>,
    body: web::Json<UpdateCategoryDto>,
) -> impl Responder {
    let id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap().clone();
    let service = state.category_service(claims.company_id);

    match service.update(id, body.into_inner(), &claims.userid).await {
        Ok(_) => ApiResponse::response(
            (), 
            Some("Category updated successfully".to_string()), 
            actix_web::http::StatusCode::OK
        ),
        Err(e) => ApiResponse::error(
            Some(e.to_string()), 
            actix_web::http::StatusCode::NOT_FOUND
        ),
    }
}