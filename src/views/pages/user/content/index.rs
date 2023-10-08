#![allow(non_snake_case)]
use leptos::*;
use crate::views::Loading;
// use leptos_router::*;

#[component]
pub fn UserIndex() -> impl IntoView {
    view! {
        <div class="h-full">
            <Loading/>
        </div>
    }
}