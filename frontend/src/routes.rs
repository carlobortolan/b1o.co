use yew::{html, Html, virtual_dom::AttrValue};
use yew_router::Routable;

use crate::pages::{watch_detail::WatchDetail, watch_list::WatchList};

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Watches,
    #[at("/watches/:watch_id")]
    WatchDetail { watch_id: String },
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        #[allow(clippy::let_unit_value)]
        Route::Watches => {
            html! { <WatchList /> }
        }
        Route::WatchDetail { watch_id } => {
            html! {<WatchDetail watch_id={AttrValue::Owned(watch_id.to_owned())} />}
        }
        Route::NotFound => html! { <h2>{"404! Not found!"}</h2>},
    }
}
