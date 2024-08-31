use leptos::*;

mod components;
mod routes;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <components::Header />
        <routes::Home />
        <components::Footer />
    }
}
