use yew::prelude::*;
use yewdux::use_store;
use super::card::Card;
use crate::State;

#[function_component]
pub fn Cards() -> Html {
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

