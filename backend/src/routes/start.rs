use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};
use database::{sea_orm::DbErr, PlayerQueries};

use crate::config::app_state::AppState;

pub async fn start(_req: HttpRequest, data: Data<AppState>) -> impl Responder {
    let db = &data.db;

    log::info!("Requested game",);

    let players = PlayerQueries::find_random_pair(db).await;

    match players {
        Ok(players) => HttpResponse::Ok().json(players),
        Err(err) => handle_internal_error(err),
    }
}

fn handle_internal_error(err: DbErr) -> HttpResponse {
    log::info!("ERR: {}", err.to_string());
    HttpResponse::InternalServerError().json(err.to_string())
}
