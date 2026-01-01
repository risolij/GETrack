use yew::prelude::*;
use super::super::components::{
    navbar::NavBar,
    hero::Hero,
    cards::Cards
};

#[function_component]
pub fn HomePage() -> Html {
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

