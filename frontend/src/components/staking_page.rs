use yew::prelude::*;

#[function_component(StakingPage)]
pub fn staking_page() -> Html {
    html! {
        <div>
            <h2>{ "Staking" }</h2>
            <p>{ "Stake your Foxtrot tokens to support the network and earn rewards." }</p>
        </div>
    }
}
