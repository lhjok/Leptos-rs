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
pub use self::error::UserError;

use leptos::*;
use leptos_router::*;
use crate::views::Loading;
use crate::api::{
    NormRes, OnlyCookie,
    UserInfoRes
};
use gloo_storage::{
    LocalStorage, Storage
};

#[component]
pub fn User() -> impl IntoView {
    let (info, set_info) = create_signal(None::<UserInfoRes>);
    provide_context(info);
    view! {{
        match LocalStorage::get("login") {
            Ok(user) => {
                let _norm: NormRes = user;
                let get_info = move || OnlyCookie::new();
                let results = create_resource(get_info,
                    move |get: OnlyCookie| async move {
                        match get.user_info().await {
                            Ok(res) => {
                                set_info.set(Some(res.clone()));
                                Some(res)
                            },
                            Err(_) => {
                                LocalStorage::delete("login");
                                None
                            }
                        }
                    }
                );
                view! { { move ||
                    match results.get() {
                        None => view! {
                            <Loading/>
                        }.into_view(),
                        Some(result) => {
                            match result {
                                None => view! {
                                    <Redirect path="/login"/>
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
                    <Redirect path="/login"/>
                }.into_view()
            }
        }
    }}
}