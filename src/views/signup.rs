#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;
use leptonic::prelude::*;
use crate::api::UserInfo;

// 服务器请求地址
const URL: &'static str = "http://127.0.0.1:3000";

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    // 填写表单的信号
    let (name, set_name) = create_signal(cx, "".to_owned());
    let (pass, set_pass) = create_signal(cx, "".to_owned());
    let (email, set_email) = create_signal(cx, "".to_owned());
    let (phone, set_phone) = create_signal(cx, "".to_owned());
    let (wait, set_wait) = create_signal(cx, false);
    // 创建一个异步活动。
    let action = create_action( cx,
        move |(name, pass, email, phone):
            &(String, String, String, String)| {
        // 将结构反序列成Json格式
        let user = UserInfo {
            username: name.to_string(), password: pass.to_string(),
            mail: email.to_string(), phone: phone.to_string(),
            ..Default::default()
        };
        // 返回异步闭包
        async move {
            set_wait.update(|w| *w = true);
            let result = user.signin(URL).await;
            set_wait.update(|w| *w = false);
            match result {
                Ok(res) => {
                    if res.status == "1" {
                        let navigate = use_navigate(cx);
                        _ = navigate("/login", Default::default());
                    } else {
                        log!("注册失败: {}", res.message);
                    }
                },
                Err(err) => {
                    log!("注册失败: {}", err);
                }
            }
        }
    });
    // 按下后按钮为不可激活状态
    let disabled = Signal::derive(cx, move || wait.get());
    // 执行按钮按下后，提交表单函数。
    let dispatch = move || action.dispatch((name.get(), pass.get(), email.get(), phone.get()));
    // 执行按钮按下后，按钮点击状态函数。
    let button_disabled = Signal::derive(cx, move || {
        disabled.get() || pass.get().is_empty() || name.get().is_empty() ||
        email.get().is_empty() || phone.get().is_empty()
    });
    view! { cx,
        <section class="h-screen">
            <div class="container h-full px-6 py-24 mx-auto max-w-7xl">
                <div class="g-6 flex h-full flex-wrap items-center justify-center lg:justify-between">
                    <div class="mb-12 md:mb-0 md:w-8/12 lg:w-6/12">
                        <img src="/static/images/draw2.webp" class="w-full" alt="Phone image" />
                    </div>

                    <div class="md:w-8/12 lg:ml-6 lg:w-5/12">
                        <form on:submit=|event| event.prevent_default()>
                            <Stack orientation=StackOrientation::Vertical spacing=Size::Em(1.2)>
                                <TextInput get=name set=set_name placeholder="name *"/>
                                <PasswordInput get=pass set=set_pass placeholder="password *"/>
                                <TextInput get=email set=set_email placeholder="email"/>
                                <TextInput get=phone set=set_phone placeholder="phone *"/>
                            </Stack>

                            <button type="submit" data-te-ripple-init data-te-ripple-color="light"
                                class="inline-block mt-5 w-full items-center justify-center btn-secondary"
                                prop:disabled=move || button_disabled.get()
                                on:click=move |_| dispatch()>"注册"
                            </button>

                            <div class="my-3 flex items-center before:mt-0.5 before:flex-1 before:border-t \
                                before:border-neutral-300 after:mt-0.5 after:flex-1 after:border-t after:border-neutral-300">
                                <p class="mx-4 my-0 text-center font-semibold dark:text-neutral-200">"OR"</p>
                            </div>

                            <A href="/login">
                                <button class="flex w-full items-center justify-center btn-secondary">
                                    "用户登录"
                                </button>
                            </A>
                        </form>
                    </div>
                </div>
            </div>
        </section>
    }
}