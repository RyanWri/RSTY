use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <ul>
                    <li><a href="/">"Home"</a></li>
                    <li><a href="/route1">"Route 1"</a></li>
                    <li><a href="/route2">"Route 2"</a></li>
                    <li><a href="/route3">"Route 3"</a></li>
                </ul>
            </nav>
            <main>
                <Route path="/" view=|| view! { <h1>"Home Page"</h1><button>"Go Home"</button> } />
                <Route path="/route1" view=|| view! { <h1>"Route 1"</h1><button>"Go to Route 1"</button> } />
                <Route path="/route2" view=|| view! { <h1>"Route 2"</h1><button>"Go to Route 2"</button> } />
                <Route path="/route3" view=|| view! { <h1>"Route 3"</h1><button>"Go to Route 3"</button> } />
            </main>
        </Router>
    }
}
