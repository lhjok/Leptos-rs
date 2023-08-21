#![allow(non_snake_case)]
use leptos::*;
// use leptos_router::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="header">
            <h1>"Header"</h1>
        </div>
    }
}
