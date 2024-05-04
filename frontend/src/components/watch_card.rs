use entity::watch::Model as Watch;
use yew::{classes, function_component, html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub watch_handle: UseStateHandle<Option<Watch>>,
    pub class: &'static str,
}

#[function_component(WatchCard)]
pub fn watch_card(props: &Props) -> Html {
    let watch = &*props.watch_handle;
    let class = props.class;

    if let Some(watch) = &watch {
        html! {
          <div class={classes!("card", "shadow-sm", class)}  >
             <div class="row g-0">
                 <div class="col">
                     <div class="card-body">
                         <h6>{&watch.manufacturer}{" | "}{&watch.manufacturer_location}</h6>
                         <div class="d-flex align-items-baseline">
                            <h5 class="card-title">{&watch.name}{" ["}</h5>
                            <h6>{"ref."}{&watch.reference}</h6>
                            <h5 class="card-title">{"]"}</h5>
                         </div>
                         <p class="card-text">{"Average rating: "}{{format!("({:.2})", &watch.average_rating)}}</p>
                         <p class="card-text">{&watch.description}</p>
                     </div>
                 </div>
                 <div class="col-md-auto">
                     <img
                     src={watch.image_url.to_owned()}
                     class="card-img-right watch-img m-3"
                     alt={format!("Photo of {}", &watch.name)} />
                 </div>
             </div>
          </div>
        }
    } else {
        html! {"Loading..."}
    }
}
