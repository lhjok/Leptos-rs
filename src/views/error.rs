#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;

#[component]
pub fn Error() -> impl IntoView {
    view! {
        <div id="error">
            <h3>"404 Error"</h3>
            <p>"您访问的页面已经不存在了..."</p>
            <A href="/">"返回首页"</A>
        </div>
    }
}
