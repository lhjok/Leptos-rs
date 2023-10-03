#![allow(non_snake_case)]
pub mod index;
use leptos::*;
// use leptos_router::*;
pub use self::index::UserIndex;

#[component]
pub fn Content(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <section class="h-full pl-60 pt-14 pb-[378px]">
            { children(cx) }
        </section>
    }
}