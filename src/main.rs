use actix_web::{middleware::Logger, web, App, HttpServer};
use dirs;
use sqlx::postgres::PgPoolOptions;

mod config;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config_path = format!(
        "{home_dir}/.config/mercury.toml",
        home_dir = dirs::home_dir().unwrap().to_str().unwrap()
    );

    let app_config = config::reader::get_config(config_path).unwrap();
    let logging_value = format!(
        "actix_web={actix_level},sqlx={sqlx_level}",
        actix_level = app_config.logging_config.actix_web_log_level,
        sqlx_level = app_config.logging_config.sqlx_log_level
    );
    std::env::set_var("RUST_LOG", logging_value);
    env_logger::init();

    let db_connection_string = format!(
        "postgres://{username}:{password}@{host}:{port}/{database}",
        username = app_config.postgres.username,
        password = app_config.postgres.password,
        host = app_config.postgres.host,
        port = app_config.postgres.port,
        database = app_config.postgres.database
    );
    let pool = PgPoolOptions::new()
        .connect(&db_connection_string)
        .await
        .expect("Failed to connect to DB");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    std::env::set_var("DATABASE_URL", db_connection_string);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::JsonConfig::default())
            .data(pool.clone())
            .route("/hey", web::post().to(handlers::hello::manual_hello))
            .route(
                "/heywithdb",
                web::post().to(handlers::hello::manual_hello_with_db),
            )
    });
    let server_address = format!(
        "{host}:{port}",
        host = app_config.app_config.host,
        port = app_config.app_config.port
    );
    server.bind(server_address)?.run().await
}
