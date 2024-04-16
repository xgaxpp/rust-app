use actix_web::web;

use self::ping_controller::ping;

pub mod ping_controller;
pub mod server_metadata;
pub mod auth;

/// Initialize the api routes, all the routes are under `/api`
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(server_metadata::get_server_metadata)
            .service(ping)
            .configure(auth::init_routes)
    );
}