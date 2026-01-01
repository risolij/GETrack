use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use std::rc::Rc;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use log::info;
use wasm_bindgen_futures::spawn_local;

#[derive(Store, Default, PartialEq)]
struct State {
    mappings: Vec<Item>,
    latest: Latest,
    //five_minute: FiveMinuteData
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct LatestItem {
    high: Option<i32>,
    high_time: Option<i32>,
    low: Option<i32>,
    low_time: Option<i32>
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
struct Latest {
    data: HashMap<i32, LatestItem>
}


//#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
//#[serde(rename_all = "camelCase")]
//struct FiveMinute {
//    avg_high_price: Option<i32>,
//    high_price_volume: Option<i32>,
//    avg_low_price: Option<i32>,
//    low_price_volume: Option<i32>
//}
//
//#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
//struct FiveMinuteData {
//    data: HashMap<i32, FiveMinute>
//}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
struct Item {
    examine: String,
    id: i32,
    members: bool,
    lowalch: Option<i32>,
    limit: Option<i32>,
    value: i32,
    highalch: Option<i32>,
    icon: String,
    name: String 
}

enum Action {
    FirstLoad { mappings: Vec<Item>, latest: Latest },
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
                .json::<Latest>()
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
struct ItemProps {
    id: i32
}

#[function_component]
fn Card(props: &ItemProps) -> Html {
    let (state, _dispatch) = use_store::<State>();
    let item = state.mappings.iter().find(|item| item.id == props.id);

    if item.is_none() {
        return html! {}
    }

    let item = item.unwrap();
    let icon = item.icon.replace(" ", "_");
    let source = format!("https://oldschool.runescape.wiki/images/{}", icon);
    let price = state.latest.data.get(&item.id);

    let ul = if let Some(price) = price {
        if price.high.is_none() ||
           price.low.is_none() || 
           price.high_time.is_none() ||
           price.low_time.is_none() {
               return html! {}
        }

        html! {
            <ul>
                <li><strong>{ "High: " }</strong>{ price.high.unwrap() }</li>
                <li><strong>{ "High Timestamp: " }</strong>{ price.high_time.unwrap() }</li>
                <li><strong>{ "Low: " }</strong>{ price.low.unwrap() }</li>
                <li><strong>{ "Low Timestamp: " }</strong>{ price.low_time.unwrap() }</li>
            </ul>
        }
    } else {
        html! {
            <ul>
                <li><strong>{ "High: " }</strong>{ "No Price Data" }</li>
                <li><strong>{ "High Timestamp: " }</strong>{ "No Price Data" }</li>
                <li><strong>{ "Low: " }</strong>{ "No Price Data" }</li>
                <li><strong>{ "Low Timestamp: " }</strong>{ "No Price Data" }</li>
            </ul>
        }
    };

    html! {
        <article class="card">
            <section class="card-header">
                <span>{ &item.name }</span>
                <span>{ item.id }</span>
            </section>
            <section class="card-body">
                <img class="card-icon" src={ source } />
                <div class="card-stats">{ ul }</div>
            </section>
        </article>
    }
}

#[function_component]
fn Cards() -> Html {
    let (state, dispatch) = use_store::<State>();
    let items = state.mappings
        .iter()
        .filter(|item| item.limit.unwrap_or(0) > 0)
        .map(|item| html! { <Card id={ item.id } /> })
        .take(30)
        .collect::<Html>();

    html! {
        <div class="cards">{items}</div>
    }
}

#[function_component]
fn NavBar() -> Html {
    html! {
        <nav>
            <a href="#section1" >{ "Section 1" }</a>
            <a href="#section2" >{ "Section 2" }</a>
            <a href="#section3" >{ "Section 3" }</a>
            <a href="#section4" >{ "Section 4" }</a>
        </nav>
    }
}

#[function_component]
fn Hero() -> Html {
    html! {
        <div class="hero"></div>
    }
}

#[function_component]
fn HomePage() -> Html {
    html! {
        <>
            <NavBar />
            <Hero />
            <main>
                <Cards />
            </main>
        </>
    }
}
