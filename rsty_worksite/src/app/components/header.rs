use std::thread::Scope;

use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header>
            <h1>"My Leptos App"</h1>
        </header>
    }
}
