use leptos::*;
mod components;  // Include the components module

#[component]
fn App() -> impl IntoView {
    view! {
        <components::button::Button title="Click Me".to_string() />
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
    console_error_panic_hook::set_once();
}