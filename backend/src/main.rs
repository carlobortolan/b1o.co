use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use config::{app_state::AppState, cors::get_cors_config, database::get_db_config, env::Env};
use routes::{
    add_review::add_review, all_players::all_players, all_watches::watches,
    create_player::create_player, next_player::next_player, one_player_by_id::one_player_by_id,
    one_watch_by_id::one_watch_by_id, start::start,
};
use shared::{
    NEXT_PLAYER_ROUTE, PLAYERS_ROUTE, REVIEWS_BY_WATCH_ROUTE, SCOREBOARD_ROUTE,
    SINGLE_PLAYER_ROUTE, SINGLE_WATCH_ROUTE, START_ROUTE, WATCHES_ROUTE,
};
use std::io::Error;

pub mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();
    let env = Env::init();

    let db = get_db_config(&env)
        .await
        .map_err(|db_err| Error::new(std::io::ErrorKind::ConnectionAborted, db_err.to_string()))?;

    let state = AppState { db };

    let host = env
        .get_backend_url()
        .host()
        .expect("there to be a backend host")
        .to_string();
    let port = env
        .get_backend_url()
        .port()
        .expect("there to be a backend port");

    log::info!("Listening on {}:{}", host, port);
    HttpServer::new(move || {
        let cors = get_cors_config(&env);

        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(cors)
            .route(WATCHES_ROUTE, web::get().to(watches))
            .route(SINGLE_WATCH_ROUTE, web::get().to(one_watch_by_id))
            .route(REVIEWS_BY_WATCH_ROUTE, web::post().to(add_review))
            .route(PLAYERS_ROUTE, web::post().to(create_player))
            .route(NEXT_PLAYER_ROUTE, web::patch().to(next_player))
            .route(START_ROUTE, web::post().to(start))
            .route(SINGLE_PLAYER_ROUTE, web::get().to(one_player_by_id))
            .route(SCOREBOARD_ROUTE, web::get().to(all_players))
    })
    .bind((host, port))?
    .run()
    .await
}
