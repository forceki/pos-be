use actix_web::web;
use crate::controllers::store_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stores") 
            .service(store_controller::get_stores)
            .service(store_controller::create_store)
    );
}