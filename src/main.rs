use actix_web::{web, App, HttpServer};
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

use app_state::AppState;

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
            .configure(api::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}