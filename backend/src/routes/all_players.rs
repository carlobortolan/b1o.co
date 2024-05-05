use actix_web::{
    web::{self, Data},
    HttpRequest, HttpResponse, Responder,
};
use database::{sea_orm::DbErr, PlayerQueries};

use crate::config::app_state::AppState;
use shared::PaginationParams;

pub async fn all_players(
    _req: HttpRequest,
    data: Data<AppState>,
    queries: web::Query<PaginationParams>,
) -> impl Responder {
    let db = &data.db;
    let page = queries.0.page.unwrap_or(1);
    let per_page = queries.0.limit.unwrap_or(10);

    log::info!(
        "Requested scoreboard page {}, with {} entries per page",
        page,
        per_page
    );

    if page <= 0 || per_page <= 0 {
        return HttpResponse::BadRequest().body("Invalid page or per_page value");
    }

    let players = PlayerQueries::find_all(db, page, per_page).await;

    match players {
        Ok(players) => HttpResponse::Ok().json(players),
        Err(err) => handle_internal_error(err),
    }
}

fn handle_internal_error(err: DbErr) -> HttpResponse {
    log::info!("ERR: {}", err.to_string());
    HttpResponse::InternalServerError().json(err.to_string())
}
