#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use js_sys::JSON;
use std::rc::Rc;

#[wasm_bindgen(module="/static/scripts\
/swiper-bundle.esm.browser.min.js")]
extern {
    // 导出Swiper类型
    type Swiper;
    // 创建Swiper实例(构造函数)
    #[wasm_bindgen(constructor)]
    fn new(id: &str, init: &JsValue) -> Swiper;
    // Swipe清理事件监听器方法
    #[wasm_bindgen(method, js_name= detachEvents)]
    fn detach_events(this: &Swiper);
    // Swipe初始化方法
    #[wasm_bindgen(method)]
    fn init(this: &Swiper);
}

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // 解析Json格式到JsValue对象
    let init = JSON::parse(r#"{
        "init": false,
        "direction": "horizontal",
        "pagination": {
            "el": ".swiper-pagination"
        },
        "navigation": {
            "nextEl": ".swiper-button-next",
            "prevEl": ".swiper-button-prev"
        }
    }"#).unwrap();
    // 实例化(构造函数)
    let swiper = Rc::new(Swiper::new(".swiper", &init));
    let listener = Rc::clone(&swiper);
    let start = Rc::clone(&swiper);
    on_cleanup(cx, move || { listener.detach_events(); });
    request_animation_frame(move || { start.init(); });
    view! { cx,
        <div id="home">
            <header class="absolute w-full h-16 z-50 bg-white/20 shadow-2xl">
                <div class="flex flex-row">
                    <div class="basis-52 ml-6">
                        <img alt="Logo" src="/static/images/logo.png"/>
                    </div>
                    <div class="basis-60 ml-auto py-3">
                        <A href="signup">
                            <button class="btn-primary mx-4">"注册用户"</button>
                        </A>
                        <A href="login">
                            <button class="btn-primary">"用户登录"</button>
                        </A>
                    </div>
                </div>
            </header>
            <div class="swiper w-full">
                <div class="swiper-wrapper">
                    <div class="swiper-slide">
                        <img alt="Poster01" src="/static/images/poster01.jpg"/>
                    </div>
                    <div class="swiper-slide">
                        <img alt="Poster02" src="/static/images/poster02.jpg"/>
                    </div>
                </div>
                <div class="swiper-pagination"></div>
                <div class="swiper-button-prev"></div>
                <div class="swiper-button-next"></div>
            </div>
        </div>
    }
}