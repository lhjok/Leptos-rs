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
    NormRes, OnlyCookie
};
use gloo_storage::{
    LocalStorage, Storage
};

#[component]
pub fn Admin() -> impl IntoView {
    view! {{
        match LocalStorage::get("login") {
            Ok(user) => {
                let _norm: NormRes = user;
                let get_info = move || OnlyCookie::new();
                let results = create_resource(get_info,
                    move |get: OnlyCookie| async move {
                        match get.admin_info().await {
                            Ok(res) => Some(res),
                            Err(_) => {
                                LocalStorage::delete("login");
                                None
                            }
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
                                Some(data) => {
                                    view! {
                                        <Header info=data.clone()/>
                                        <Aside/>
                                        <Content>
                                            <Outlet/>
                                        </Content>
                                        <Footer/>
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