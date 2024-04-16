use std::env;

use actix_cors::Cors;
use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_web::middleware::Logger;
use actix_web::{
    http,
    web::{self, JsonConfig, QueryConfig},
    App, HttpRequest, HttpServer,
};
use config::db;
use errors::Error as ApiError;

mod api;
mod config;
mod errors;
mod schemas;
mod ratelimit;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Load '.env' File Error");
    pretty_env_logger::init();


    let app_host = env::var("APP_HOST").expect("APP_HOST not found");
    let app_port = env::var("APP_PORT").expect("APP_HOST not found");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    let pool = db::init_db_pool(&db_url).await;

    let ratelimit_backend = InMemoryBackend::builder().build();

    
    log::info!("{}", "The RESTful API is available at <http://{app_host}:{app_port}/api/>");

    HttpServer::new(move || {
        let ratelimit_middleware_builder = ratelimit::init_ip(ratelimit_backend.clone());

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(ratelimit_middleware_builder.build())
            .wrap(Logger::default())
            .configure(api::init_routes)
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(JsonConfig::default().error_handler(|err, _| ApiError::from(err).into()))
            .app_data(QueryConfig::default().error_handler(|err, _| ApiError::from(err).into()))
            
            .default_service(web::route().to(|req: HttpRequest| async move {
                let path = req.path();
                if path.ends_with('/') {
                    ApiError::NotFound(format!(
                        "There is no endpoint in this path with this method. Our API doesn't support trailing slashes, try `{}`",
                        path.trim_end_matches('/')
                    ))
                } else {
                    ApiError::NotFound("There is no endpoint in this path with this method".to_owned())
                }
            }))
    })
    .bind(&app_url)?
    .run()
    .await
}
