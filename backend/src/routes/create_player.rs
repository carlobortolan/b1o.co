use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use database::{sea_orm::DbErr, PlayerQueries};
use entity::player;

use crate::config::app_state::AppState;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewPlayer {
    pub name: String,
    pub image_url: String,
    pub source: String,
}

pub async fn create_player(
    data: Data<AppState>,
    new_player: web::Json<NewPlayer>,
) -> impl Responder {
    log::info!("Creating player: {:?}", new_player.0);
    let db = &data.db;

    let player_model = player::Model {
        name: new_player.name.clone(),
        image_url: new_player.image_url.clone(),
        source: new_player.source.clone(),
        ..Default::default()
    };

    let new_player: Result<player::Model, DbErr> =
        PlayerQueries::create_player(db, player_model).await;

    match new_player {
        Ok(player) => HttpResponse::Created().json(player),
        Err(err) => handle_internal_error(err),
    }
}

fn handle_internal_error(err: DbErr) -> HttpResponse {
    log::info!("ERR: {}", err.to_string());
    HttpResponse::InternalServerError().json(err.to_string())
}
