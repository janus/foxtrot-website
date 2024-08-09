use yew::prelude::*;

#[function_component(MiningPage)]
pub fn mining_page() -> Html {
    html! {
        <div>
            <h2>{ "Mining" }</h2>
            <p>{ "Participate in the Foxtrot network and earn rewards by mining." }</p>
        </div>
    }
}
