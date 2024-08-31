use leptos::*;

fn main() {
    mount_to_body(|| view! { <p>"Hello, world!"</p> });
    console_error_panic_hook::set_once();
}