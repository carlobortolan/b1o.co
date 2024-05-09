use crate::config::app_state::AppState;
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use database::{
    sea_orm::{DbConn, DbErr},
    PlayerQueries,
};
use serde::Deserialize;
use shared::NextPlayerParams;

pub async fn update_ratings(
    db: &DbConn,
    winner: i32,
    loser: i32,
) -> Result<(entity::player::ActiveModel, entity::player::ActiveModel), DbErr> {
    // Fetch the winner and loser from the database
    let winner_player = PlayerQueries::find_one(db, winner).await?;
    let loser_player = PlayerQueries::find_one(db, loser).await?;

    if let (Some(mut winner_player), Some(mut loser_player)) = (winner_player, loser_player) {
        // Log ratings before update
        let winner_rating = winner_player.rating;
        let loser_rating = loser_player.rating;

        log::info!(
            "Before update - Winner rating: {}, Loser rating: {}",
            winner_rating,
            loser_rating
        );

        // Update scores
        winner_player.calculate_rating(true, loser_rating);
        loser_player.calculate_rating(false, winner_rating);

        // Log ratings after update
        log::info!(
            "After update - Winner rating: {}, Loser rating: {}",
            winner_player.rating,
            loser_player.rating
        );

        // Save updated players back to the database
        let winner_player_am = PlayerQueries::update_player(db, winner_player.clone()).await?;
        let loser_player_am = PlayerQueries::update_player(db, loser_player.clone()).await?;

        Ok((winner_player_am, loser_player_am))
    } else {
        Err(DbErr::RecordNotFound("PlayerNotFound".to_string()))
    }
}

pub async fn next_player(
    data: Data<AppState>,
    queries: web::Query<NextPlayerParams>,
    visited_ids: web::Json<VisitedIDs>,
) -> impl Responder {
    let db = &data.db;

    let winner = queries.0.winner.try_into().unwrap();
    let loser = queries.0.loser.try_into().unwrap();

    log::info!("Scoring [Winner: {}, Loser: {}]", winner, loser);

    match update_ratings(db, winner, loser).await {
        Ok((winner, _)) => {
            // After updating scores, fetch the "next player"
            match PlayerQueries::find_next(db, winner, visited_ids.visited_ids.clone()).await {
                Ok(player) => match player {
                    Some(player) => HttpResponse::Ok().json(player),
                    None => handle_not_found(),
                },
                Err(err) => handle_internal_error(err),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to update scores"),
    }
}

fn handle_internal_error(err: DbErr) -> HttpResponse {
    log::info!("ERR: {}", err.to_string());
    HttpResponse::InternalServerError().json(err.to_string())
}

fn handle_not_found() -> HttpResponse {
    HttpResponse::NotFound().json(format!("Next player could not be found"))
}

#[derive(Deserialize, Debug)]
pub struct VisitedIDs {
    pub visited_ids: Vec<i32>,
}
