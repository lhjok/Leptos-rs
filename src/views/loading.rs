#![allow(non_snake_case)]
use leptos::*;
// use wasm_bindgen::prelude::*;
// use web_sys::Element as WebSysElement;
// use gloo::events::EventListener;
// use gloo_utils::document;
// use js_sys::JSON;

// #[wasm_bindgen(module="/node_modules\
// /tw-elements/dist/js/tw-elements.es.min.js")]
// extern {
//     type LoadingManagement;
//     #[wasm_bindgen(constructor)]
//     fn new(el: &WebSysElement, init: &JsValue) -> LoadingManagement;
//     #[wasm_bindgen(method)]
//     fn dispose(this: &LoadingManagement);
// }
//
// fn init_loading_management(id: &str, init: &JsValue) -> Result<(WebSysElement, LoadingManagement), JsValue> {
//     let trigger = document().get_element_by_id(id).unwrap();
//     let loading = LoadingManagement::new(&trigger, init);
//     let loadings = (trigger, loading);
//     Ok(loadings)
// }

#[component]
pub fn Loading(cx: Scope) -> impl IntoView {
    // 解析Json格式到JsValue对象
    // let init = JSON::parse(r#"{
    //     "parentSelector": "\u0023loading"
    // }"#).unwrap();
    // // 执行第三方JS初始化代码
    // request_animation_frame( move || {
    //     let loadings = init_loading_management("loadingTE", &init).unwrap();
    //     let (element, loading) = loadings;
    //     let event = EventListener::new(&element, "show.te.loadingManagement", move |_event| {
    //         // event.prevent_default();
    //         loading.dispose();
    //     });
    //     event.forget();
    // });
    view! { cx,
        <div class="flex h-full w-full justify-center">
            <div class="h-14 w-20 self-center">
                <div class="flex w-full pl-3">
                    <div class="inline-block h-8 w-8 animate-spin border-transparent text-success \
                        dark:text-success-400 motion-reduce:animate-[spin_1.5s_linear_infinite]" role="status">
                        <span class="[&>svg]:w-8">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 \
                                0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"/>
                            </svg>
                        </span>
                    </div>
                </div>
                <div class="text-success dark:text-success-400">Loading...</div>
            </div>
        </div>
    }
}