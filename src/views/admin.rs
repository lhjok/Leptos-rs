#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;
use crate::api::{ AdminQuery, UserName };
use gloo_storage::{ LocalStorage, Storage };
use super::{ Loading, pages::{
    Aside, Footer, Header, Content,
}};
// 服务器请求地址
const URL: &'static str = "http://127.0.0.1:3000";
#[component]
pub fn Admin(cx: Scope) -> impl IntoView {
    view! { cx, {
        match LocalStorage::get("username") {
            Ok(user) => {
                let storage: UserName = user;
                let get_user = move || storage.clone();
                let results = create_resource(cx,
                    get_user, move |name: UserName| async move {
                        let user = vec![("username", name.username)];
                        let admin = AdminQuery { user };
                        let result = admin.info(URL).await;
                        match result {
                            Ok(res) => Some(res),
                            Err(_) => {
                                LocalStorage::delete("username");
                                None
                            }
                        }
                    }
                );
                view! { cx, { move ||
                    match results.read(cx) {
                        None => view! { cx,
                            <Loading/>
                        }.into_view(cx),
                        Some(result) => {
                            match result {
                                None => view! { cx,
                                    <Redirect path="/login"/>
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
            },
            Err(_) => {
                view! { cx,
                    <Redirect path="/login"/>
                }.into_view(cx)
            }
        }
    }}
}