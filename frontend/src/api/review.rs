use std::error::Error;

use entity::review;
use reqwasm::http::Request;
use shared::REVIEWS_BY_WATCH_ROUTE;

use super::get_api_url;

pub async fn add_review(
    watch_id: i32,
    review: &review::Model,
) -> Result<review::Model, Box<dyn Error>> {
    let endpoint = REVIEWS_BY_WATCH_ROUTE.replace("{watch_id}", &watch_id.to_string());
    let url = get_api_url(&endpoint);
    let as_js_value =
        serde_wasm_bindgen::to_value(review).expect("the review to be translatable to a jsvalue");
    let as_string = js_sys::JSON::stringify(&as_js_value).expect("the jsvalue to be stringafyable");

    Request::post(url.as_str())
        .header("Content-Type", "application/json")
        .header("Origin", "https://ticktack.carlobortolan.com") // Replace with your frontend URL
        .header("Referer", "https://ticktack.carlobortolan.com") // Replace with your frontend URL
        .body(as_string)
        .send()
        .await?
        .json::<review::Model>()
        .await
        .map_err(|err| err.into())
}
