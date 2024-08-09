use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::console;

mod components;
mod router;

use router::{switch, AppRoute};

#[function_component(App)]
fn app() -> Html {
    console::log_1(&"Rendering App component".into());
    html! {
        <BrowserRouter>
            <components::Header />
            <components::Navigation />
            <Switch<AppRoute> render={switch} />
            <components::Footer />
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();
    console::log_1(&"Starting Yew app".into());
    yew::Renderer::<App>::new().render();
}
