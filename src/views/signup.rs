#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;
use leptonic::prelude::*;
// use wasm_bindgen::prelude::*;
// use web_sys::Element as WebSysElement;
// use gloo::events::EventListener;
// use gloo_utils::document;

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "".to_owned());
    let (pass, set_pass) = create_signal(cx, "".to_owned());
    let (email, set_email) = create_signal(cx, "".to_owned());
    let (phone, set_phone) = create_signal(cx, "".to_owned());
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
                                class="inline-block mt-5 w-full items-center justify-center btn-secondary">"注册"
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