use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

/// Enishalize the database connection pool, return the database connection and if the database is existed.
/// ### Panics
/// * If can't get the database url from the environment
/// * If the database connection pool cannot be created
pub async fn init_db_pool(url: &str) -> DatabaseConnection {

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        // .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

    Database::connect(opt)
    .await
    .expect("Failed tocreate database connectio pool")
}
