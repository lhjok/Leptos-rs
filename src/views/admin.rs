#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;
use crate::api::{ AdminQuery, UserName, AdminInfoRes };
use gloo_storage::{ LocalStorage, Storage };
use super::{ LoginErr, Loading };
use super::pages::{
    Aside, Footer, Header, Content
};
// 服务器请求地址
const URL: &'static str = "http://127.0.0.1:3000";
#[component]
pub fn Admin(cx: Scope) -> impl IntoView {
    let (info, set_info) = create_signal(cx, None::<AdminInfoRes>);
    match LocalStorage::get("username") {
        Ok(user) => {
            let local_storage: UserName = user;
            let get_user = move || local_storage.clone();
            let results = create_resource(cx,
                get_user, move |name: UserName| async move {
                    let user = vec![("username", name.username)];
                    let admin = AdminQuery { user };
                    let result = admin.info(URL).await;
                    match result {
                        Ok(res) => {
                            log!("登录成功");
                            set_info.set(Some(res));
                        },
                        Err(err) => {
                            log!("服务器已断开: {}", err);
                            LocalStorage::delete("username");
                        }
                    }
                    info.get()
                }
            );
            view! {cx, { move ||
                match results.read(cx) {
                    None => view! {cx,
                        <Loading/>
                    }.into_view(cx),
                    Some(result) => {
                        match result {
                            None => view! { cx,
                                <LoginErr href="/login"/>
                            }.into_view(cx),
                            Some(_data) => {
                                view! { cx,
                                    <Header/>
                                    <Aside/>
                                    <Content>
                                        <Outlet/>
                                    </Content>
                                    <Footer/>
                                }.into_view(cx)
                            }
                        }
                    }
                }
            }}.into_view(cx)
        }
        Err(err) => {
            log!("您还没有登录: {}", err);
            view! { cx,
                <LoginErr href="/login"/>
            }.into_view(cx)
        }
    }
}