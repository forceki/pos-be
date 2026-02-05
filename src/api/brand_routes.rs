use actix_web::web;
use crate::{controllers::brand_controllers, middleware::auth_middleware::JwtMiddleware};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/brand") 
            .wrap(JwtMiddleware)
            .service(brand_controllers::get_brands)
            .service(brand_controllers::create_brand)
            .service(brand_controllers::update_brand)
    );
}