#![allow(non_snake_case)]
use leptos::*;
// use leptos_router::*;

#[component]
pub fn UserError(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="user-error">
            <h1>"User Error"</h1>
        </div>
    }
}