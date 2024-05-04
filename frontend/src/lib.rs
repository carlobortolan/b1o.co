mod api;
mod components;
mod pages;
mod routes;

use routes::{switch, Route};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{function_component, html};
use yew_router::{BrowserRouter, Switch};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn main() {
    println!("STARTED main 0");
    yew::start_app::<TickTack>();
    println!("STARTED main 1");
}

#[function_component(TickTack)]
pub fn rate_watch() -> Html {
    println!("STARTED rate_watch");

    html! {
        <div class="container mt-5">
          <h1>{ "Rate your favorite watches" }</h1>
          <div>
            <BrowserRouter>
              <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
          </div>
        </div>
    }
}


