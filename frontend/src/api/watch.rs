use super::get_api_url;
use entity::watch::Model as Watch;
use entity::review::Model as Review;
use reqwasm::http::Request;
use shared::{ApiQueryParams, WATCHES_ROUTE};
use std::error::Error;

pub async fn get_watches() -> Result<Vec<Watch>, Box<dyn Error>> {
    println!("STARTED get_watches");

    let url = get_api_url(WATCHES_ROUTE);

    let watches = Request::get(url.as_str())
        .header("Origin", "https://ticktack.carlobortolan.com") // Replace with your frontend URL
        .header("Referer", "https://ticktack.carlobortolan.com") // Replace with your frontend URL
        .send()
        .await?
        .json::<Vec<Watch>>()
        .await?;

    Ok(watches)
}

pub async fn get_watch(
    id: i32,
    queries: Option<ApiQueryParams>,
) -> Result<(Watch, Vec<Review>), Box<dyn Error>> {
    let mut url = get_api_url(&[WATCHES_ROUTE, &id.to_string()].join("/"));

    if let Some(queries) = queries {
        if let Some(expand) = &queries.expand {
            url.set_query(Some(&format!("expand={}", expand)))
        }
    }

    let watch = Request::get(url.as_str())
        .header("Origin", "https://ticktack.carlobortolan.com") // Replace with your frontend URL
        .header("Referer", "https://ticktack.carlobortolan.com") // Replace with your frontend URL
        .send()
        .await?
        .json::<(Watch, Vec<Review>)>()
        .await?;

    Ok(watch)
}
