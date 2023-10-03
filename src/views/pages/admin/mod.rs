#![allow(non_snake_case)]
pub mod aside;
pub mod content;
pub mod footer;
pub mod header;
pub mod error;

pub use self::aside::Aside;
pub use self::content::Content;
pub use self::footer::Footer;
pub use self::header::Header;
pub use self::error::AdminError;

use leptos::*;
use leptos_router::*;
use crate::views::Loading;
use crate::api::{ GetQuery, UserName };
use gloo_storage::{ LocalStorage, Storage };
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
                        let get = GetQuery { user };
                        let result = get.admin_info(URL).await;
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
                                    <Redirect path="/logins"/>
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
                    <Redirect path="/logins"/>
                }.into_view(cx)
            }
        }
    }}
}