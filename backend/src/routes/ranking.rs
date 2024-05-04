use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};
use database::WatchQueries;

use crate::config::app_state::AppState;

pub async fn watches(_req: HttpRequest, data: Data<AppState>) -> impl Responder {
    let db = &data.db;
        let watches = WatchQueries::find_all(db).await;

    match watches {
        Ok(watches) => HttpResponse::Ok().json(watches),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}
