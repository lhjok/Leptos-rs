#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;

#[component]
pub fn LoginErr(cx: Scope, href: &'static str) -> impl IntoView {
    view! { cx,
        <div class="px-6 py-20 text-center text-neutral-800">
            <h1 class="mb-3 text-5xl font-bold">您还没有登录</h1>
            <h3 class="mb-8 text-3xl font-bold">请返回登录页面</h3>
            <A href={href}>
                <button class="btn-primary">"用户登录"</button>
            </A>
        </div>
    }
}