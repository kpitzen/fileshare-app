use actix_web::{ App, HttpServer, web, middleware::Logger };
use sqlx::postgres::PgPoolOptions;
use dirs;

mod config;
mod handlers;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config_path = format!(
        "{home_dir}/.config/fileshare-app.toml",
        home_dir = dirs::home_dir().unwrap().to_str().unwrap()
    );

    let app_config = config::reader::get_config(config_path).unwrap();

    let db_connection_string = format!(
        "postgres://{username}:{password}@{host}:{port}/{database}",
        username=app_config.postgres.username,
        password=app_config.postgres.password,
        host=app_config.postgres.host,
        port=app_config.postgres.port,
        database=app_config.postgres.database
    );
    let pool = PgPoolOptions::new()
        .connect(&db_connection_string).await.expect(
            "Failed to connect to DB"
        );
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await.expect(
            "Failed to run migrations"
        );

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let server = HttpServer::new(||
        App::new()
            .wrap(Logger::default())
            .app_data(web::JsonConfig::default())
            .route("/hey",
                web::post().to(
                    handlers::hello::manual_hello
                )
            )
    );
    let server_address = format!(
        "{host}:{port}",
        host=app_config.app_config.host,
        port=app_config.app_config.port
    );
    server.bind(server_address)?
        .run()
        .await
}
