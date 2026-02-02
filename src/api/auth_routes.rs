use actix_web::web;
use crate::controllers::auth_controllers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth") 
            .service(auth_controllers::onboard)
            .service(auth_controllers::login)
    );
}