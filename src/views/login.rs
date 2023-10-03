#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use web_sys::Element as WebSysElement;
use gloo::events::EventListener;
use gloo_utils::document;
use gloo_storage::{ LocalStorage, Storage };
use leptonic::prelude::*;
use crate::api::{ User, UserName };

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

// 服务器请求地址
const URL: &'static str = "http://127.0.0.1:3000";

#[component]
pub fn Login(cx: Scope, role: &'static str) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "".to_owned());
    let (pass, set_pass) = create_signal(cx, "".to_owned());
    let (wait, set_wait) = create_signal(cx, false);
    let action = create_action(cx, move |(name, pass): &(String, String)| {
        let username = name.to_string();
        let password = pass.to_string();
        let local_storage = UserName { username: username.clone() };
        let user = User { username, password };
        async move {
            set_wait.update(|w| *w = true);
            let result = user.login(URL, role).await;
            set_wait.update(|w| *w = false);
            match result {
                Ok(res) => {
                    if res.status == "1" {
                        LocalStorage::set("username", local_storage)
                            .expect("LocalStorage::set");
                        let navigate = use_navigate(cx);
                        _ = navigate(&format!("/{}/index", role), Default::default());
                    }
                }
                Err(err) => {
                    log!("登录失败: {}", err);
                }
            }
        }
    });
    // 按下后按钮为不可激活状态
    let disabled = Signal::derive(cx, move || wait.get());
    // 执行按钮按下后，提交表单函数。
    let dispatch = move || action.dispatch((name.get(), pass.get()));
    // 执行按钮按下后，按钮点击状态函数。
    let button_disabled = Signal::derive(cx, move || {
        disabled.get() || pass.get().is_empty() || name.get().is_empty()
    });
    // 执行第三方JS初始化代码
    request_animation_frame( move || {
        let dropdowns = init_dropdown("[data-te-dropdown-toggle-ref]").unwrap();
        for (element, dropdown) in dropdowns.into_iter() {
            let event = EventListener::new(&element, "click", move |_event| {
                // event.prevent_default();
                dropdown.toggle();
            });
            event.forget();
        }
    });
    view! { cx,
        <section class="h-full">
            <div class="container h-full px-6 py-24 mx-auto max-w-7xl">
                <div class="g-6 flex h-full flex-wrap items-center justify-center lg:justify-between">
                    <div class="mb-12 md:mb-0 md:w-8/12 lg:w-6/12">
                        <img src="/static/images/draw2.webp" class="w-full" alt="login"/>
                    </div>

                    <div class="md:w-8/12 lg:ml-6 lg:w-5/12">
                        <form on:submit=|event| event.prevent_default()>
                            <Stack orientation=StackOrientation::Vertical spacing=Size::Em(1.2)>
                                <TextInput get=name set=set_name placeholder="UserName"/>
                                <PasswordInput get=pass set=set_pass placeholder="Password"/>
                                <div class="mb-6 flex w-full items-center justify-between">
                                    <div class="mb-[0.125rem] block min-h-[1.5rem] pl-[1.5rem]">
                                        <input class="c-input" type="checkbox" value="" id="exampleCheck3" checked/>
                                        <label class="inline-block pl-[0.15rem] hover:cursor-pointer" for="exampleCheck3">
                                            "记住帐号"
                                        </label>
                                    </div>
                                    <Link href="/">"您忘记密码了吗?"</Link>
                                </div>
                            </Stack>

                            <button data-te-ripple-init data-te-ripple-color="light"
                                class="inline-block w-full items-center justify-center btn-secondary"
                                prop:disabled=move || button_disabled.get()
                                on:click=move |_| dispatch()>"登录"
                            </button>

                            <div class="my-4 flex items-center before:mt-0.5 before:flex-1 before:border-t \
                                before:border-neutral-300 after:mt-0.5 after:flex-1 after:border-t after:border-neutral-300">
                                <p class="mx-4 my-0 text-center font-semibold dark:text-neutral-200">"OR"</p>
                            </div>

                            <A href="/">
                                <button class="mb-5 flex w-full items-center justify-center btn-secondary">
                                    "返回首页"
                                </button>
                            </A>

                            <div class="relative" data-te-dropdown-ref>
                                <button type="button" id="dropdownMenuButton1"
                                    class="flex w-full items-center justify-center btn-secondary"
                                    style="background-color: #55acee" data-te-dropdown-toggle-ref
                                    aria-expanded="false">"导航菜单"
                                    <span class="ml-2 w-2">
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5">
                                            <path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 \
                                            111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z" clip-rule="evenodd"/>
                                        </svg>
                                    </span>
                                </button>
                                <ul class="ul-menu w-full [&[data-te-dropdown-show]]:block"
                                    aria-labelledby="dropdownMenuButton1" data-te-dropdown-menu-ref>
                                    <li class="li-menu text-center"><A href="/signup">"注册用户"</A></li>
                                    <li class="li-menu text-center"><A href="/admin">"管理后台"</A></li>
                                </ul>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </section>
    }
}