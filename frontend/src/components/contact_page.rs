use yew::prelude::*;

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
    html! {
        <div>
            <h2>{ "Contact" }</h2>
            <p>{ "Get in touch with the Foxtrot team for more information." }</p>
        </div>
    }
}
