#![allow(non_snake_case)]
use leptos::*;
// use leptos_router::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="footer">
            <p>"千鸟科技 - 瑞金市千鸟网络科技有限公司 CopyRight 2017-2020"
            <span>" - 赣ICP备17011754号-2 - "</span>" 软件著作权 - "
            <span>"软著登字第8888888号"</span></p>
        </div>
    }
}