use crate::config::app_state::AppState;
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use database::{sea_orm::DbErr, PlayerQueries};

pub async fn one_player_by_id(path: web::Path<i32>, data: Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let db = &data.db;
    log::info!("Requested player with id {}", id);

    match PlayerQueries::find_one(db, id).await {
        Ok(player) => match player {
            Some(player) => HttpResponse::Ok().json(player),
            None => handle_not_found(id),
        },
        Err(err) => handle_internal_error(err),
    }
}

fn handle_internal_error(err: DbErr) -> HttpResponse {
    log::info!("ERR: {}", err.to_string());
    HttpResponse::InternalServerError().json(err.to_string())
}

fn handle_not_found(id: i32) -> HttpResponse {
    HttpResponse::NotFound().json(format!("Player with ID {} not found", id))
}
