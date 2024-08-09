use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <p>{ "© 2024 Foxtrot Blockchain" }</p>
        </footer>
    }
}
