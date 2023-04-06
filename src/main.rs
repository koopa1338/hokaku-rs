use leptos::*;
use leptos_meta::*;

#[component]
pub fn Root(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! { cx,
        "TODO"
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,  <Root/> })
}
