use crate::{api::watch::get_watches, routes::Route};
use std::cmp::Ordering;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html, MouseEvent};
use yew_router::history::History;
use yew_router::prelude::use_history;
use itertools::Itertools;
use entity::watch::Model;

#[function_component(WatchList)]
pub fn watch_list() -> Html {
    #[allow(clippy::redundant_closure)]
        let watches = use_state(|| Vec::new());
    let has_error = use_state(|| false);
    let history = use_history().expect("history to be available");
    let row_click = |id: i32| -> Callback<MouseEvent> {
        let history = history.clone();
        Callback::once(move |_: yew::MouseEvent| {
            history.push(Route::WatchDetail {
                watch_id: id.to_string(),
            })
        })
    };

    let sort_column = use_state(|| "".to_string());
    let sort_direction = use_state(|| "".to_string());

    if *has_error {
        history.push(Route::NotFound)
    }

    {
        let watches = watches.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let result = get_watches().await;
                    match result {
                        Ok(resp) => watches.set(resp),
                        // We could check the error and respond differently.
                        Err(_) => has_error.set(true),
                    };
                });
                || ()
            },
            (),
        );
    }

    let sortable_header_click = |column: &'static str| -> Callback<MouseEvent> {
        let sort_column = sort_column.clone();
        let sort_direction = sort_direction.clone();
        Callback::once(move |_| {
            let current_sort_column = (*sort_column).clone();
            let current_sort_direction = (*sort_direction).clone();

            let new_sort_direction = if current_sort_column == String::from(column) {
                // Toggle the sort direction if the same column is clicked
                if current_sort_direction == "asc" {
                    "desc".to_string()
                } else {
                    "asc".to_string()
                }
            } else {
                // Set the sort direction to ascending by default for a new column
                "asc".to_string()
            };

            sort_column.set(column.to_string());
            sort_direction.set(new_sort_direction);
        })
    };

    html! {
        <div class="mt-4 w-75 mx-auto">
            <div class="d-flex flex-column">
                <h1>{ "Watch List" }</h1>
                <table class="table shadow-sm">
                    <colgroup>
                        <col width="45%" />
                        <col width="20%" />
                        <col width="10%" />
                        <col width="25%" />
                    </colgroup>
                    <thead>
                        <tr>
                            <th class="pointer" scope="col" onclick={sortable_header_click("name")}>{"Name"}</th>
                            <th class="pointer" scope="col" onclick={sortable_header_click("reference")}>{"Reference"}</th>
                            <th class="pointer" scope="col" onclick={sortable_header_click("average_rating")}>{"Rating"}</th>
                            <th class="pointer" scope="col" onclick={sortable_header_click("style")}>{"Style"}</th>
                        </tr>
                    </thead>
                    <tbody>
                    {
                        watches.clone().iter().map(|watch| {

                        html!{
                            <tr class="pointer" onclick={row_click(watch.id)}>
                                <th scope="row">{&watch.name}</th>
                                <td >{&watch.reference}</td>
                                <td >{format!("{:.2}", &watch.average_rating)}</td>
                                <td >{&watch.style}</td>
                            </tr>
                        }
                    }).collect::<Html>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
