use yew::prelude::*;

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <nav>
            <a href="#section1" >{ "Section 1" }</a>
            <a href="#section2" >{ "Section 2" }</a>
            <a href="#section3" >{ "Section 3" }</a>
            <a href="#section4" >{ "Section 4" }</a>
        </nav>
    }
}

