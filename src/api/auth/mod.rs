use actix_web::web;

pub mod login;
pub mod utils;
pub mod traits;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login::login)
    );
}