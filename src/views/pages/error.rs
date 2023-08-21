#![allow(non_snake_case)]
use leptos::*;
// use leptos_router::*;

#[component]
pub fn AdminError(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="admin-error">
            <h1>"Admin Error"</h1>
        </div>
    }
}