use actix_web::web;
use crate::{controllers::roles_controllers, middleware::auth_middleware::JwtMiddleware};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/roles") 
            .wrap(JwtMiddleware)
            .service(roles_controllers::index)
            .service(roles_controllers::create)
    );
}