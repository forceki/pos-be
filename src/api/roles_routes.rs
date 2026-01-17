use actix_web::web;
use crate::controllers::roles_controllers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/roles") 
            .service(roles_controllers::create)
    );
}