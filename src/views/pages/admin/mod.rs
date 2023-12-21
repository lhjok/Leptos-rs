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
use crate::api::{ 
    NormRes, OnlyCookie,
    AdminInfoRes, AdminInfo
};
use gloo_storage::{
    LocalStorage, Storage
};

#[component]
pub fn Admin() -> impl IntoView {
    let (info, set_info) = create_signal(None::<AdminInfoRes>);
    provide_context(info);
    view! {{
        match LocalStorage::get("login") {
            Ok(user) => {
                let _norm: NormRes = user;
                let get_info = move || OnlyCookie::new();
                let results = create_resource(get_info,
                    move |get: OnlyCookie| async move {
                        match get.admin_info().await {
                            Ok(res) => {
                                set_info.set(Some(res.clone()));
                                Some(res)
                            },
                            Err(_) => {
                                // 调试时模拟数据
                                let info = AdminInfo {
                                    id: 1,
                                    mail: "lhjok@live.cn".to_owned(),
                                    username: "admin".to_owned(),
                                    password: "12345678".to_owned(),
                                    phone: "13387073000".to_owned(),
                                    avatar: "/static/images/admin/admin-IQX2kk-avatar.jpg".to_owned(),
                                    status: 1
                                };
                                let res = AdminInfoRes {
                                    data: info,
                                    status: "1".to_owned(),
                                    message: "成功获得信息".to_owned()
                                };
                                set_info.set(Some(res.clone()));
                                Some(res)
                            }
                            // Err(_) => {
                            //     LocalStorage::delete("login");
                            //     None
                            // }
                        }
                    }
                );
                view! {{ move ||
                    match results.get() {
                        None => view! {
                            <Loading/>
                        }.into_view(),
                        Some(result) => {
                            match result {
                                None => view! {
                                    <Redirect path="/logins"/>
                                }.into_view(),
                                Some(_) => {
                                    view! {
                                        <Header/>
                                        <Aside/>
                                        <Content>
                                            <Outlet/>
                                            <Footer/>
                                        </Content>
                                    }.into_view()
                                }
                            }
                        }
                    }
                }}.into_view()
            },
            Err(_) => {
                view! {
                    <Redirect path="/logins"/>
                }.into_view()
            }
        }
    }}
}