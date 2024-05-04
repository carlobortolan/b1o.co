use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use database::WatchQueries;
use entity::review;

use crate::config::app_state::AppState;

pub async fn add_review(
    path: web::Path<i32>,
    data: Data<AppState>,
    rating: web::Json<review::Model>,
) -> impl Responder {
    println!("HIT ADD REVIEW ----------------------------------------");
    let watch_id = path.into_inner();
    let db = &data.db;
    let new_review = WatchQueries::add_review(db, watch_id, rating.0).await;

    match new_review {
        Ok(review) => HttpResponse::Created().json(review),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
