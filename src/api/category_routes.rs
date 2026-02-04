use actix_web::web;

use crate::{controllers::category_controllers, middleware::auth_middleware::JwtMiddleware};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .wrap(JwtMiddleware)
            .service(category_controllers::get_all)
            .service(category_controllers::get_category_tree)   
            .service(category_controllers::create_category)
            .service(category_controllers::update_category)
    );
}