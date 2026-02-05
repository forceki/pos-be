use actix_web::web;
use crate::{controllers::store_controllers, middleware::auth_middleware::JwtMiddleware};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stores") 
            .wrap(JwtMiddleware)
            .service(store_controllers::get_stores)
            .service(store_controllers::create_store)
            .service(store_controllers::update_store)
            .service(store_controllers::archive_store)
    );
}