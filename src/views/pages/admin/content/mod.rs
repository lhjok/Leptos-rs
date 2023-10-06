#![allow(non_snake_case)]
pub mod index;
pub mod signin;

use leptos::*;
// use leptos_router::*;
pub use self::index::AdminIndex;
pub use self::signin::AdminSignup;

#[component]
pub fn Content(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <section class="h-full pl-60 pt-14 pb-[378px]">
            { children(cx) }
        </section>
    }
}