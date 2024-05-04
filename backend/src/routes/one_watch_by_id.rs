use crate::config::app_state::AppState;
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use database::{sea_orm::DbErr, WatchQueries};
use entity::watch::Relation;
use shared::ApiQueryParams;
use std::str::FromStr;

pub async fn one_watch_by_id(
    path: web::Path<i32>,
    data: Data<AppState>,
    queries: web::Query<ApiQueryParams>,
) -> impl Responder {
    let id = path.into_inner();
    let db = &data.db;
    log::info!("Requested watch with id {}", id);

    if let Some(expand) = &queries.expand {
        let relation = Relation::from_str(expand);
        match relation {
            Ok(relation) => match WatchQueries::find_with_related(db, id, relation).await {
                Ok(watch) => match &watch.0 {
                    Some(_) => HttpResponse::Ok().json(watch),
                    None => handle_not_found(id),
                },
                Err(err) => handle_internal_error(err),
            },
            Err(_) => handle_bad_request(BadRequestType::InvalidRelation(expand)),
        }
    } else {
        match WatchQueries::find_one(db, id).await {
            Ok(watch) => match watch {
                Some(watch) => HttpResponse::Ok().json(watch),
                None => handle_not_found(id),
            },
            Err(err) => handle_internal_error(err),
        }
    }
}

fn handle_internal_error(err: DbErr) -> HttpResponse {
    log::info!("ERR: {}", err.to_string());
    HttpResponse::InternalServerError().json(err.to_string())
}

fn handle_not_found(id: i32) -> HttpResponse {
    HttpResponse::NotFound().json(format!("Watch with id {} not found", id))
}

enum BadRequestType<'a> {
    InvalidRelation(&'a String),
}

fn handle_bad_request(request_type: BadRequestType) -> HttpResponse {
    match request_type {
        BadRequestType::InvalidRelation(relation) => HttpResponse::BadRequest().json(format!(
            "{} is not a valid relation for the requested entity",
            relation
        )),
    }
}
