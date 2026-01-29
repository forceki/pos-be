use actix_web::web;
use crate::controllers::users_controllers;
use crate::middleware::auth_middleware::JwtMiddleware;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users") 
            .wrap(JwtMiddleware)
            .service(users_controllers::get_users)
            .service(users_controllers::get_users_by_id)
    );
}
