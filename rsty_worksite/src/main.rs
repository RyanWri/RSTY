use leptos::*;

#[component]
fn Home() -> impl IntoView {
    view! {
        <h1>"Hello, Leptos! (Home Page)"</h1>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h1>"Hello, Leptos! (About Page)"</h1>
    }
}

#[component]
fn App() -> impl IntoView {
    let current_path = window().location().pathname().unwrap();

    view! {
        <>
            <nav>
                <a href="/">"Home"</a>
                <a href="/about">"About"</a>
            </nav>
            <main>
                {
                    if current_path == "/" {
                        view! { <Home /> }.into_view()
                    } else if current_path == "/about" {
                        view! { <About /> }.into_view()
                    } else {
                        view! { <h1>"404 - Not Found"</h1> }.into_view()
                    }
                }
            </main>
        </>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
}
