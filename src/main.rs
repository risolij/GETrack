use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use std::rc::Rc;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use log::info;
use wasm_bindgen_futures::spawn_local;
use pages::homepage::HomePage;
use models::{
    item::Item,
    latest::LatestSales
};

mod components;
mod pages;
mod models;

#[derive(Store, Default, PartialEq)]
struct State {
    mappings: Vec<Item>,
    latest: LatestSales,
}

enum Action {
    FirstLoad { mappings: Vec<Item>, latest: LatestSales },
}

impl Reducer<State> for Action {
    fn apply(self, state: Rc<State>) -> Rc<State> {
        match self {
            Action::FirstLoad { mappings, latest } => {
                State { mappings, latest }.into()
            }
        }
    }
}

#[function_component]
fn App() -> Html {
    let (state, dispatch) = use_store::<State>();
    let client = reqwest::Client::builder()
        .user_agent("Creating GeTracker Appliction")
        .build()
        .expect("Failed to build client");

    use_effect_with((), move |_| {
        spawn_local(async move {
            log::info!("API Request to Mapping");
            let items = client.get("https://prices.runescape.wiki/api/v1/osrs/mapping")
                .send()
                .await
                .expect("Failed to make mapping request")
                .json::<Vec<Item>>()
                .await
                .expect("Failed to parse into rust mappings");
        
            log::info!("API Request to Latest");
            let latest = client.get("https://prices.runescape.wiki/api/v1/osrs/latest")
                .send()
                .await
                .expect("Failed to make 5m request")
                .json::<LatestSales>()
                .await
                .expect("Failed to parse into rust prices");
        
            dispatch.apply(Action::FirstLoad {
                mappings: items,
                latest: latest
            });
        });

        || ()
    });

    html! {
        <BrowserRouter>
            <Switch<Routes> render={switch} />
        </BrowserRouter>
    }
    
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Initializing yew..");
    yew::Renderer::<App>::new().render();
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::NotFound => html! { <h1>{ "404" }</h1> },
        Routes::Home => html! { <HomePage /> }
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq)]
pub struct ItemProps {
    id: i32
}
