use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div>
            <h2>{ "Welcome to Foxtrot Blockchain" }</h2>
            <p>{ "Explore the future of decentralized finance with unparalleled privacy and security." }</p>
        </div>
    }
}
