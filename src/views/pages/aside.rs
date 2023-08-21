#![allow(non_snake_case)]
use leptos::*;
// use leptos_router::*;

#[component]
pub fn Aside(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="aside">
            <h1>"Aside"</h1>
        </div>
    }
}