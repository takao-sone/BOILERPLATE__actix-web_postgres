use std::env;

use actix_web::http::header;
use actix_web::{middleware, App, HttpServer};
use env_logger;

use boilerplate::api;
use boilerplate::config::{
    create_cors_config, create_csrf_config, create_redis_session, get_app_bound_address,
    get_database_url, get_frontend_origin, get_is_test_environment, get_redis_address,
    get_redis_cookie_session_private_key, get_rust_backtrace, get_rust_log, get_valid_origin_value,
    get_valid_referer_value, set_rust_logs,
};
use boilerplate::db::connection::new_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok(); // In order to use a .env file.
    env_logger::init();

    // Command Line Args
    let command_line_args: Vec<String> = env::args().collect();
    let is_test_environment = get_is_test_environment(command_line_args);
    // Environmental Variables
    let database_url = get_database_url(is_test_environment);
    let cors_allowed_origin = get_frontend_origin();
    let csrf_valid_referer_value = get_valid_referer_value();
    let csrf_valid_origin_value = get_valid_origin_value();
    let app_bound_address = get_app_bound_address(is_test_environment);
    let rust_log = get_rust_log();
    let rust_backtrace = get_rust_backtrace();
    set_rust_logs(rust_log, rust_backtrace);

    // DB & Connection Pooling
    let pool = new_pool(database_url, is_test_environment).expect("Failed to create pool.");

    // Main Server
    HttpServer::new(move || {
        let redis_session = create_redis_session(
            get_redis_address(is_test_environment),
            get_redis_cookie_session_private_key(is_test_environment).as_bytes(),
            60 * 60 * 24 * 3, // 3 days
            "session",
            actix_redis::SameSite::Lax,
            time::Duration::days(3),
            true,
            false,
        );
        let cors_config = create_cors_config(
            cors_allowed_origin.clone(),
            vec!["GET", "POST", "DELETE"],
            vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE],
            3600, // 1 hour
        );
        let csrf_config = create_csrf_config(
            csrf_valid_origin_value.clone(),
            csrf_valid_referer_value.clone(),
        );

        App::new()
            .data(pool.clone())
            .wrap(redis_session)
            .wrap(cors_config)
            .wrap(csrf_config)
            .wrap(middleware::Logger::default())
            .configure(api::api_factory)
    })
    .bind(app_bound_address)?
    .run()
    .await
}
