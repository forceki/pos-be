use actix_web::web;
use crate::controllers::company_controllers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/company") 
            .service(company_controllers::create)
    );
}