use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <div>
            <h2>{ "About Foxtrot" }</h2>
            <p>{ "Foxtrot Blockchain offers confidentiality and performance improvements over traditional blockchains." }</p>
        </div>
    }
}
