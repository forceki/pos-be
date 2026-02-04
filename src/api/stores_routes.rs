use actix_web::web;
use crate::{controllers::store_controller, middleware::auth_middleware::JwtMiddleware};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stores") 
            .wrap(JwtMiddleware)
            .service(store_controller::get_stores)
            .service(store_controller::create_store)
            .service(store_controller::update_store)
            .service(store_controller::archive_store)
    );
}