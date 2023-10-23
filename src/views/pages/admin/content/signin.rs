#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;
use leptos::logging::log;
use leptonic::prelude::*;
use crate::api::AdminInfo;

// 服务器请求地址
const URL: &'static str = "http://127.0.0.1:3000";

#[component]
pub fn AdminSignup() -> impl IntoView {
    // 填写表单的信号
    let (name, set_name) = create_signal("".to_owned());
    let (pass, set_pass) = create_signal("".to_owned());
    let (email, set_email) = create_signal("".to_owned());
    let (phone, set_phone) = create_signal("".to_owned());
    let (wait, set_wait) = create_signal(false);
    // 创建一个异步活动。
    let action = create_action(
        move |(name, pass, email, phone):
            &(String, String, String, String)| {
            // 将结构序列成Json格式
            let user = AdminInfo {
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
                            let navigate = use_navigate();
                            navigate("/login", Default::default());
                        } else {
                            log!("添加子帐号失败: {}", res.message);
                        }
                    },
                    Err(err) => {
                        log!("添加子帐号失败: {}", err);
                    }
                }
            }
        });
    // 按下后按钮为不可激活状态
    let disabled = Signal::derive(move || wait.get());
    // 执行按钮按下后，提交表单函数。
    let dispatch = move || action.dispatch((name.get(), pass.get(), email.get(), phone.get()));
    // 执行按钮按下后，按钮点击状态函数。
    let button_disabled = Signal::derive(move || {
        disabled.get() || pass.get().is_empty() || name.get().is_empty() ||
            email.get().is_empty() || phone.get().is_empty()
    });
    view! {
        <div class="mt-20 md:w-8/12 lg:ml-24 lg:w-4/12">
            <form on:submit=|event| event.prevent_default()>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Em(1.2)>
                    <TextInput get=name set=set_name placeholder="name *"/>
                    <PasswordInput get=pass set=set_pass placeholder="password *"/>
                    <TextInput get=email set=set_email placeholder="email"/>
                    <TextInput get=phone set=set_phone placeholder="phone *"/>
                </Stack>
                <button type="submit" data-te-ripple-init data-te-ripple-color="light"
                    class="inline-block mt-8 w-full items-center justify-center btn-secondary"
                    prop:disabled=move || button_disabled.get()
                    on:click=move |_| dispatch()>"添加子帐号"
                </button>
            </form>
        </div>
    }
}