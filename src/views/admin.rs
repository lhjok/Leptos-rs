#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;
use crate::api::{ AdminQuery, UserName };
use gloo_storage::{ LocalStorage, Storage };
use super::pages::{
    Aside, Footer, Header, Content
};

#[component]
pub fn Admin(cx: Scope) -> impl IntoView {
    if let Ok(user) = LocalStorage::get("username") {
        let local_storage: UserName = user;
        let get_user = move || local_storage.clone();
        let results = create_resource( cx,
            get_user, move |name: UserName| async move {
                let username = name.username.as_str();
                let user = vec![("username", username)];
                let admin = AdminQuery { user };
                let result = admin.info("http://127.0.0.1:3000").await;
                match result {
                    Ok(res) => {
                        if res.status == "1" {
                            log!("登录成功");
                        } else {
                            log!("登录失败！请重新登录。");
                            let navigate = use_navigate(cx);
                            let _ = navigate("/login", Default::default());
                        }
                    }
                    Err(err) => {
                        log!("登录失败(2): {}", err);
                    }
                }
            }
        );
        results.read(cx);
    } else {
        let navigate = use_navigate(cx);
        let _ = navigate("/login", Default::default());
    }
    view! { cx,
        <Header/>
        <Aside/>
        <Content>
            <Outlet/>
        </Content>
        <Footer/>
    }
}