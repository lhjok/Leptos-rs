#![allow(non_snake_case)]
pub mod index;
use leptos::*;
// use leptos_router::*;
pub use self::index::UserIndex;

#[component]
pub fn Content(children: Children) -> impl IntoView {
    view! {
        <section class="h-full pl-60 pt-14 pb-[378px]">
            { children() }
        </section>
    }
}