use leptos::prelude::*;
fn main() {
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world! From Bogdan"</p> });
    console_error_panic_hook::set_once();
}
