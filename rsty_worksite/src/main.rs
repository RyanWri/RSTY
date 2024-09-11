use leptos::*;
use leptos_router::*;
use log::Level;

mod components;

#[component]
fn App() -> impl IntoView {
    let button_title: String = "Click Me RSTY!".to_string();
    view! {
        <Router>
            <Route path="/" view=move || view! { <components::button::Button title={button_title.clone()} /> } />
            <Route path="/sound" view=|| view! { <components::sound::SoundPage /> } />
        </Router>
    }
}

fn main() {
    console_log::init_with_level(Level::Debug).expect("error initializing log");
    console_error_panic_hook::set_once(); // Logs panics to the browser's console
    mount_to_body(App);
}
