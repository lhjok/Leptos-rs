#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="login">
            <div class="basis-60 ml-auto py-3">
                <button class="btn-primary mx-4">
                    <A href="/" class="text-base">"返回首页"</A>
                </button>
                <button class="btn-primary">
                    <A href="/amdin" class="text-base">"登录后台"</A>
                </button>
            </div>
        </div>
    }
}
