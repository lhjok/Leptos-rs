#![allow(non_snake_case)]
pub mod index;
use leptos::*;
// use leptos_router::*;
pub use self::index::AdminIndex;

#[component]
pub fn Content(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div id="content" class="h-4/5">
            { children(cx) }
        </div>
    }
}