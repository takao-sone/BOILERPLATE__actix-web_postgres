use std::env;

use actix_web::http::HeaderName;

use crate::app_middleware::csrf;

pub fn get_is_test_environment(command_line_args: Vec<String>) -> bool {
    command_line_args.len() > 1 && command_line_args[1] == "test"
}

pub fn get_database_url(is_test_environment: bool) -> String {
    if is_test_environment {
        return env::var("TEST_DB_URL").expect("Failed to get TEST_DB_URL.");
    }

    env::var("DB_URL").expect("Failed to get DB_URL.")
}

pub fn get_redis_address(is_test_environment: bool) -> String {
    if is_test_environment {
        return env::var("TEST_REDIS_ADDRESS").expect("Failed to get TEST_REDIS_ADDRESS.");
    }

    env::var("REDIS_ADDRESS").expect("Failed to get REDIS_ADDRESS.")
}

pub fn get_redis_cookie_session_private_key(is_test_environment: bool) -> String {
    if is_test_environment {
        return env::var("TEST_REDIS_COOKIE_SESSION_PRIVATE_KEY")
            .expect("Failed to get TEST_REDIS_COOKIE_SESSION_PRIVATE_KEY.");
    }

    env::var("REDIS_COOKIE_SESSION_PRIVATE_KEY")
        .expect("Failed to get REDIS_COOKIE_SESSION_PRIVATE_KEY.")
}

pub fn get_frontend_origin() -> String {
    env::var("FRONTEND_ORIGIN").expect("Failed to get FRONTEND_ORIGIN.")
}

pub fn get_valid_referer_value() -> String {
    env::var("VALID_REFERER_VALUE").expect("Failed to get VALID_REFERER_VALUE.")
}

pub fn get_valid_origin_value() -> String {
    env::var("VALID_ORIGIN_VALUE").expect("Failed to get VALID_ORIGIN_VALUE.")
}

pub fn get_app_bound_address(is_test_environment: bool) -> String {
    if is_test_environment {
        return env::var("TEST_APP_BOUND_ADDRESS").expect("Failed to get TEST_APP_BOUND_ADDRESS.");
    }

    env::var("APP_BOUND_ADDRESS").expect("Failed to get APP_BOUND_ADDRESS.")
}

pub fn create_redis_session(
    redis_address: String,
    cookie_session_private_key: &[u8],
    session_live_time_in_seconds: u32,
    cookie_name: &str,
    cookie_same_site_policy: actix_redis::SameSite,
    cookie_max_age: time::Duration,
    is_cookie_http_only: bool,
    is_cookie_secure: bool,
) -> actix_redis::RedisSession {
    let custom_cache_key_generation_strategy = Box::new(|key: &str| format!("{}", &key));

    actix_redis::RedisSession::new(redis_address, cookie_session_private_key)
        .ttl(session_live_time_in_seconds) // 3 days
        .cookie_name(cookie_name)
        .cookie_same_site(cookie_same_site_policy)
        // .cache_keygen(Box::new(|key: &str| format!("{}", &key)))
        .cache_keygen(custom_cache_key_generation_strategy)
        .cookie_max_age(cookie_max_age)
        .cookie_http_only(is_cookie_http_only)
        // FIXME: When in production, it has to be TRUE.
        .cookie_secure(is_cookie_secure)
    // .cookie_secure(true),
}

pub fn create_cors_config(
    cors_allowed_origin: String,
    access_control_allow_methods: Vec<&str>,
    access_control_allow_headers: Vec<HeaderName>,
    access_control_max_age_in_seconds: impl Into<Option<usize>>,
) -> actix_cors::Cors {
    actix_cors::Cors::default()
        .allowed_origin(&cors_allowed_origin)
        .allowed_methods(access_control_allow_methods)
        .allowed_headers(access_control_allow_headers)
        .max_age(access_control_max_age_in_seconds)
        .supports_credentials()
}

pub fn create_csrf_config(
    csrf_valid_origin_value: String,
    csrf_valid_referer_value: String,
) -> csrf::CSRF {
    csrf::CSRF::new(csrf_valid_origin_value, csrf_valid_referer_value)
}
