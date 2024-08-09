use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::AppRoute;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <nav>
            <ul>
                <li><Link<AppRoute> to={AppRoute::Home}>{ "Home" }</Link<AppRoute>></li>
                <li><Link<AppRoute> to={AppRoute::About}>{ "About" }</Link<AppRoute>></li>
                <li><Link<AppRoute> to={AppRoute::Wallet}>{ "Wallet" }</Link<AppRoute>></li>
                <li><Link<AppRoute> to={AppRoute::Mining}>{ "Mining" }</Link<AppRoute>></li>
                <li><Link<AppRoute> to={AppRoute::Staking}>{ "Staking" }</Link<AppRoute>></li>
                <li><Link<AppRoute> to={AppRoute::Community}>{ "Community" }</Link<AppRoute>></li>
                <li><Link<AppRoute> to={AppRoute::Contact}>{ "Contact" }</Link<AppRoute>></li>
            </ul>
        </nav>
    }
}
