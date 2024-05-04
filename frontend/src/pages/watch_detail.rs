use crate::components::watch_card::WatchCard;
use crate::components::review_form::ReviewForm;
use crate::components::reviews_card::ReviewsCard;
use crate::{api::watch::get_watch, routes::Route};
use entity::watch::Relation;
use shared::ApiQueryParams;
use yew::{
    function_component, html, use_effect_with_deps, use_state, virtual_dom::AttrValue, Properties,
};
use yew_router::history::History;
use yew_router::prelude::use_history;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub watch_id: AttrValue,
}

#[function_component(WatchDetail)]
pub fn watch_detail(props: &Props) -> Html {
    let watch_id = props
        .watch_id
        .parse::<i32>()
        .expect("the watch id to be parseable to an integer.");
    let has_error = use_state(|| false);
    let watch_handle = use_state(|| None);
    #[allow(clippy::redundant_closure)]
    let reviews_handle = use_state(|| Vec::new());
    let history = use_history().expect("history to be available");

    if *has_error {
        history.push(Route::NotFound)
    }

    {
        let watch_handle = watch_handle.clone();
        let reviews_handle = reviews_handle.clone();
        let handle_dep = reviews_handle.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let queries = ApiQueryParams {
                        expand: Some(Relation::Review.to_string()),
                    };

                    let result = get_watch(watch_id, Some(queries)).await;

                    match result {
                        Ok(result) => {
                            watch_handle.set(Some(result.0));
                            reviews_handle.set(result.1);
                        }
                        Err(_) => has_error.set(true),
                    }
                });
                || ()
            },
            handle_dep,
        );
    }

    html! {
        <div class="my-4 w-75 mx-auto">
          <div class="d-flex flex-column">
           <WatchCard class="mb-3" watch_handle={watch_handle} />
           <ReviewsCard class="mb-3" reviews_handle={reviews_handle.clone()} />
           <ReviewForm watch_id={watch_id} reviews_handle={reviews_handle} />
          </div>
        </div>
    }
}
