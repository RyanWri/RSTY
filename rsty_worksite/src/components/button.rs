use leptos::*;

/// Button component that takes a title as a prop
#[component]
pub fn Button(title: String) -> impl IntoView {
    view! {
        <button>{title}</button>
    }
}
