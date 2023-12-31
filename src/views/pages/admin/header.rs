#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use web_sys::Element as WebSysElement;
use gloo::events::EventListener;
use gloo_utils::document;
use gloo_storage::{
    LocalStorage, Storage
};
use crate::api::{
    NormRes, OnlyCookie,
    AdminInfoRes
};

#[wasm_bindgen(module="/node_modules\
/tw-elements/dist/js/tw-elements.es.min.js")]
extern {
    type Dropdown;
    #[wasm_bindgen(constructor)]
    fn new(el: &WebSysElement) -> Dropdown;
    #[wasm_bindgen(method)]
    fn toggle(this: &Dropdown);
}

fn init_dropdown(id: &str) -> Result<Vec<(WebSysElement, Dropdown)>, JsValue> {
    let trigger = document().query_selector_all(id)?;
    let mut dropdowns: Vec<(WebSysElement, Dropdown)> = Vec::new();
    for i in 0..trigger.length() {
        let element = trigger.item(i).unwrap().unchecked_into();
        let dropdown = Dropdown::new(&element);
        dropdowns.push((element, dropdown));
    }
    Ok(dropdowns)
}

#[component]
pub fn Header() -> impl IntoView {
    let info = use_context::<ReadSignal<Option<AdminInfoRes>>>().unwrap();
    let (_out, set_out) = create_signal(None::<NormRes>);
    let action = create_action(move |_| async move {
        let admin = OnlyCookie::new();
        let result = admin.singout("admin").await;
        match result {
            Ok(res) => {
                set_out.set(Some(res));
                let navigate = use_navigate();
                LocalStorage::delete("login");
                navigate("/logins", Default::default());
            },
            Err(_) => {
                let navigate = use_navigate();
                LocalStorage::delete("login");
                navigate("/logins", Default::default());
            }
        }
    });
    // 点击事件调用退出函数
    let dispatch = move || {
        match LocalStorage::get("login") {
            Ok(user) => {
                let norm: NormRes = user;
                action.dispatch(norm.data);
            },
            Err(_) => {
                let navigate = use_navigate();
                navigate("/logins", Default::default());
            }
        }
    };
    // 执行第三方JS初始化代码
    request_animation_frame(move || {
        let dropdowns = init_dropdown("[data-te-dropdown-toggle-ref]").unwrap();
        for (element, dropdown) in dropdowns.into_iter() {
            let event = EventListener::new(&element, "click", move |_event| {
                // event.prevent_default();
                dropdown.toggle();
            });
            event.forget();
        }
    });
    view! {
        <nav class="fixed z-[9999] flex-no-wrap flex w-full items-center justify-between bg-[#FBFBFB] \
        py-2 shadow-md shadow-black/5 dark:bg-neutral-600 dark:shadow-black/10 lg:flex-wrap lg:justify-start lg:py-4">
            <div class="flex w-full flex-wrap items-center justify-between px-3">
                <button class="block border-0 bg-transparent px-2 text-neutral-500 hover:no-underline \
                hover:shadow-none focus:no-underline focus:shadow-none focus:outline-none focus:ring-0 \
                dark:text-neutral-200 lg:hidden" type="button" data-te-collapse-init data-te-target="#navbarSupportedContent1"
                aria-controls="navbarSupportedContent1" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="[&>svg]:w-7">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="h-7 w-7">
                            <path fill-rule="evenodd" d="M3 6.75A.75.75 0 013.75 6h16.5a.75.75 0 010 1.5H3.75A.75.75 0 013 \
                            6.75zM3 12a.75.75 0 01.75-.75h16.5a.75.75 0 010 1.5H3.75A.75.75 0 013 12zm0 5.25a.75.75 0 \
                            01.75-.75h16.5a.75.75 0 010 1.5H3.75a.75.75 0 01-.75-.75z" clip-rule="evenodd"/>
                        </svg>
                    </span>
                </button>

                <div class="!visible hidden flex-grow basis-[100%] items-center lg:!flex lg:basis-auto"
                    id="navbarSupportedContent1" data-te-collapse-item>
                    <a class="mb-4 ml-2 mr-5 mt-3 flex items-center text-neutral-900 hover:text-neutral-900 focus:text-neutral-900 \
                    dark:text-neutral-200 dark:hover:text-neutral-400 dark:focus:text-neutral-400 lg:mb-0 lg:mt-0" href="#">
                        <img src="https://tecdn.b-cdn.net/img/logo/te-transparent-noshadows.webp" style="height: 15px" alt="Logo" loading="lazy"/>
                    </a>
                    <ul class="list-style-none mr-auto flex flex-col pl-0 lg:flex-row" data-te-navbar-nav-ref>
                        <li class="mb-4 lg:mb-0 lg:pr-2" data-te-nav-item-ref>
                            <a class="text-neutral-500 transition duration-200 hover:text-neutral-700 hover:ease-in-out focus:text-neutral-700 \
                            disabled:text-black/30 motion-reduce:transition-none dark:text-neutral-200 dark:hover:text-neutral-300 \
                            dark:focus:text-neutral-300 lg:px-2 [&.active]:text-black/90 dark:[&.active]:text-zinc-400"
                                href="#" data-te-nav-link-ref>千鸟科技</a>
                        </li>
                        <li class="mb-4 lg:mb-0 lg:pr-2" data-te-nav-item-ref>
                            <a class="text-neutral-500 transition duration-200 hover:text-neutral-700 hover:ease-in-out focus:text-neutral-700 \
                            disabled:text-black/30 motion-reduce:transition-none dark:text-neutral-200 dark:hover:text-neutral-300 \
                            dark:focus:text-neutral-300 lg:px-2 [&.active]:text-black/90 dark:[&.active]:text-neutral-400"
                                href="#" data-te-nav-link-ref>团队</a>
                        </li>
                        <li class="mb-4 lg:mb-0 lg:pr-2" data-te-nav-item-ref>
                            <a class="text-neutral-500 transition duration-200 hover:text-neutral-700 hover:ease-in-out focus:text-neutral-700 \
                            disabled:text-black/30 motion-reduce:transition-none dark:text-neutral-200 dark:hover:text-neutral-300 \
                            dark:focus:text-neutral-300 lg:px-2 [&.active]:text-black/90 dark:[&.active]:text-neutral-400"
                                href="#" data-te-nav-link-ref>项目</a>
                        </li>
                    </ul>
                </div>

                <div class="relative flex items-center">
                    <a class="mr-4 text-neutral-600 transition duration-200 hover:text-neutral-700 hover:ease-in-out focus:text-neutral-700 \
                    disabled:text-black/30 motion-reduce:transition-none dark:text-neutral-200 dark:hover:text-neutral-300 \
                    dark:focus:text-neutral-300 [&.active]:text-black/90 dark:[&.active]:text-neutral-400" href="#">
                        <span class="[&>svg]:w-5">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="h-5 w-5">
                                <path d="M2.25 2.25a.75.75 0 000 1.5h1.386c.17 0 .318.114.362.278l2.558 9.592a3.752 3.752 0 00-2.806 3.63c0 \
                                .414.336.75.75.75h15.75a.75.75 0 000-1.5H5.378A2.25 2.25 0 017.5 15h11.218a.75.75 0 00.674-.421 60.358 \
                                60.358 0 002.96-7.228.75.75 0 00-.525-.965A60.864 60.864 0 005.68 4.509l-.232-.867A1.875 1.875 0 003.636 \
                                2.25H2.25zM3.75 20.25a1.5 1.5 0 113 0 1.5 1.5 0 01-3 0zM16.5 20.25a1.5 1.5 0 113 0 1.5 1.5 0 01-3 0z"/>
                            </svg>
                        </span>
                    </a>

                    <div class="relative" data-te-dropdown-ref data-te-dropdown-alignment="end">
                        <a class="hidden-arrow mr-4 flex items-center text-neutral-600 transition duration-200 hover:text-neutral-700 \
                        hover:ease-in-out focus:text-neutral-700 disabled:text-black/30 motion-reduce:transition-none dark:text-neutral-200 \
                        dark:hover:text-neutral-300 dark:focus:text-neutral-300 [&.active]:text-black/90 dark:[&.active]:text-neutral-400"
                            href="#" id="dropdownMenuButton1" role="button" data-te-dropdown-toggle-ref aria-expanded="false">
                            <span class="[&>svg]:w-5">
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="h-5 w-5">
                                    <path fill-rule="evenodd" d="M5.25 9a6.75 6.75 0 0113.5 0v.75c0 2.123.8 4.057 2.118 5.52a.75.75 0 01-.297 \
                                    1.206c-1.544.57-3.16.99-4.831 1.243a3.75 3.75 0 11-7.48 0 24.585 24.585 0 01-4.831-1.244.75.75 0 \
                                    01-.298-1.205A8.217 8.217 0 005.25 9.75V9zm4.502 8.9a2.25 2.25 0 104.496 0 25.057 25.057 0 01-4.496 0z"
                                    clip-rule="evenodd"/>
                                </svg>
                            </span>
                            <span class="absolute -mt-4 ml-2.5 rounded-full bg-danger px-[0.35em] py-[0.15em] text-[0.6rem] font-bold \
                            leading-none text-white">0</span>
                        </a>

                        <ul class="absolute z-[1000] float-left m-0 hidden min-w-max list-none overflow-hidden rounded-lg border-none \
                        bg-white bg-clip-padding text-left text-base shadow-lg dark:bg-neutral-700 [&[data-te-dropdown-show]]:block"
                            aria-labelledby="dropdownMenuButton1" data-te-dropdown-menu-ref>
                            <li>
                                <a class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 \
                                hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none \
                                disabled:bg-transparent disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
                                    href="#" data-te-dropdown-item-ref>未读消息</a>
                            </li>
                            <li>
                                <a class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 \
                                hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none \
                                disabled:bg-transparent disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
                                    href="#" data-te-dropdown-item-ref>消息管理</a>
                            </li>
                            <li>
                                <a class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 \
                                hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none \
                                disabled:bg-transparent disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
                                    href="#" data-te-dropdown-item-ref>发布工单</a>
                            </li>
                        </ul>
                    </div>

                    <div class="relative" data-te-dropdown-ref data-te-dropdown-alignment="end">
                        <a class="hidden-arrow flex items-center whitespace-nowrap transition duration-150 ease-in-out motion-reduce:transition-none"
                            href="#" id="dropdownMenuButton2" role="button" data-te-dropdown-toggle-ref aria-expanded="false">
                            <img src=move || info.get().unwrap().data.avatar class="rounded-full"
                                style="height: 25px; width: 25px" alt="" loading="lazy"/>
                        </a>
                        <ul class="absolute z-[1000] float-left m-0 hidden min-w-max list-none overflow-hidden rounded-lg border-none bg-white \
                        bg-clip-padding text-left text-base shadow-lg dark:bg-neutral-700 [&[data-te-dropdown-show]]:block"
                            aria-labelledby="dropdownMenuButton2" data-te-dropdown-menu-ref>
                            <li>
                                <A href="config">
                                    <a class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 \
                                    hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none disabled:bg-transparent \
                                    disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
                                        href="#" data-te-dropdown-item-ref>系统设置</a>
                                </A>
                            </li>
                            <li>
                                <a class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 \
                                hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none disabled:bg-transparent \
                                disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
                                    href="#" data-te-dropdown-item-ref>会员信息</a>
                            </li>
                            <li>
                                <a class="block w-full whitespace-nowrap bg-transparent px-4 py-2 text-sm font-normal text-neutral-700 \
                                hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none disabled:bg-transparent \
                                disabled:text-neutral-400 dark:text-neutral-200 dark:hover:bg-white/30"
                                    href="javascript:void(0);" on:click=move |_| dispatch() data-te-dropdown-item-ref>退出登录</a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </nav>
    }
}