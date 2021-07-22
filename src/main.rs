extern crate dotenv;

use std::env;

use actix_web::{middleware, App, HttpServer};
use boilerplate::api;
use boilerplate::db::connection::new_pool;
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=error,warn,info,debug");
    env_logger::init();
    dotenv::dotenv().ok();

    // Command line args (for Test)
    let command_line_args: Vec<String> = env::args().collect();
    let run_environment = if command_line_args.len() > 1 {
        &command_line_args[1]
    } else {
        "dev"
    };

    // DB & Connection Pooling
    let pool = new_pool(run_environment).expect("Failed to create pool.");

    // Bound address
    let bound_address = if run_environment == "test" {
        env::var("TEST_BOUND_ADDRESS").expect("Failed to get test bound address.")
    } else {
        env::var("DEV_BOUND_ADDRESS").expect("Failed to get dev bound address.")
    };

    // Main Server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // .wrap(
            //     Cors::default()
            //         .allowed_origin(&allowed_origin)
            //         .allowed_methods(vec!["GET", "POST", "DELETE"])
            //         .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //         .allowed_header(header::CONTENT_TYPE)
            //         .supports_credentials()
            //         .max_age(3600),
            // )
            // .wrap(
            //     RedisSession::new(redis_addr_port.as_str(), &private_key.as_bytes())
            //         .ttl(60 * 60 * 24 * 3)
            //         .cookie_name("session")
            //         .cookie_same_site(SameSite::None)
            //         .cache_keygen(Box::new(|key: &str| format!("{}", &key)))
            //         .cookie_max_age(time::Duration::days(3))
            //         .cookie_http_only(true)
            //         .cookie_secure(true),
            // )
            .wrap(middleware::Logger::default())
            .configure(api::api_factory)
    })
    .bind(bound_address)?
    .run()
    .await
}
