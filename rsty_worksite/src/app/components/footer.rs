use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer>
            <p>"© 2024 My Leptos Project"</p>
        </footer>
    }
}
