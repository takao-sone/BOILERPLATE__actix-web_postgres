use crate::api::path::ApiPath;
use actix_web::{web, HttpResponse};

async fn index() -> HttpResponse {
    HttpResponse::Ok().json("Hello!!!!!")
}

pub fn app_factory(app: &mut web::ServiceConfig) {
    let base_path = ApiPath {
        prefix: String::from(""),
    };

    app.route(&base_path.define(String::from("/")), web::get().to(index));
}
