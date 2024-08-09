use yew::prelude::*;

#[function_component(WalletPage)]
pub fn wallet_page() -> Html {
    html! {
        <div>
            <h2>{ "Wallet" }</h2>
            <p>{ "Manage your Foxtrot assets with ease and security." }</p>
        </div>
    }
}
