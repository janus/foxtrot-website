use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    AboutPage, CommunityPage, ContactPage, HomePage, MiningPage, StakingPage, WalletPage,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/wallet")]
    Wallet,
    #[at("/mining")]
    Mining,
    #[at("/staking")]
    Staking,
    #[at("/community")]
    Community,
    #[at("/contact")]
    Contact,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <HomePage /> },
        AppRoute::About => html! { <AboutPage /> },
        AppRoute::Wallet => html! { <WalletPage /> },
        AppRoute::Mining => html! { <MiningPage /> },
        AppRoute::Staking => html! { <StakingPage /> },
        AppRoute::Community => html! { <CommunityPage /> },
        AppRoute::Contact => html! { <ContactPage /> },
    }
}
