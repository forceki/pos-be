use actix_web::web;
use crate::{controllers::company_controllers, middleware::auth_middleware::JwtMiddleware};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/company") 
            .wrap(JwtMiddleware)
            .service(company_controllers::create)
    );
}