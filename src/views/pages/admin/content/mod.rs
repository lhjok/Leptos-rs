#![allow(non_snake_case)]
pub mod index;
pub mod signin;
pub mod config;

use leptos::*;
// use leptos_router::*;
pub use self::index::AdminIndex;
pub use self::signin::AdminSignup;
pub use self::config::AdminConfig;

#[component]
pub fn Content(children: Children) -> impl IntoView {
    view! {
        <section class="overflow-auto h-full pl-60 pt-14">
            { children() }
        </section>
    }
}
