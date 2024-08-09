use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <h1>{ "Foxtrot Blockchain" }</h1>
        </header>
    }
}
