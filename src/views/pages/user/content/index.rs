#![allow(non_snake_case)]
use leptos::*;
use crate::views::Loading;
// use leptos_router::*;

#[component]
pub fn UserIndex(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="h-full">
            <Loading/>
        </div>
    }
}