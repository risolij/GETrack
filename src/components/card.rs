use yew::prelude::*;
use crate::{ ItemProps, State };
use yewdux::use_store;

#[function_component]
pub fn Card(props: &ItemProps) -> Html {
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

