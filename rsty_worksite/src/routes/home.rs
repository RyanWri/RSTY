use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <h2>"Welcome to the Home Page!"</h2>
            <p>"This is the homepage of your Leptos project."</p>
        </div>
    }
}
