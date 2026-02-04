use actix_web::web;

mod auth_routes;
mod users_routes;
mod roles_routes;
mod company_routes;
mod stores_routes;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(auth_routes::config)
            .configure(users_routes::config)
            .configure(roles_routes::config)
            .configure(company_routes::config)

    );
}