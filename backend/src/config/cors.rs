use actix_cors::Cors;
use actix_web::http::{self, Method};

use super::env::Env;

pub fn get_cors_config(env: &Env) -> Cors {
    let mut url = env.get_frontend_url().to_string();
    if url.ends_with('/') {
        url.pop();
    }

    Cors::default()
        .allowed_origin(&url)
        // TODO: Workaround for CORS-Frontend issue
        .allowed_origin("https://ticktack.carlobortolan.com")
        .allowed_methods(vec![Method::GET, Method::POST])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
