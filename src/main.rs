use actix_web::{App, HttpResponse, HttpServer, error, web};
use sqlx::MySqlPool;
use dotenv::dotenv;
use std::env;

mod controllers;
mod dtos;
mod models;
mod repository;
mod services;
mod utils;
mod app_state;
mod api;
mod middleware;

use app_state::AppState;

use crate::utils::api_response::ApiResponse;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    let state = web::Data::new(AppState::new(pool.clone()));

    println!("🚀 Server running at 127.0.0.1:8080");


    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(
                web::JsonConfig::default().error_handler(|err, _req| {
                    let error_message = err.to_string();
                    
                    let error_response = ApiResponse::error(
                        Some(error_message),
                        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    );

                    error::InternalError::from_response(err, error_response).into()
                }),
            )
            .configure(api::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}