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
use crate::api::OnlyCookie;
// 服务器请求地址
const URL: &'static str = "http://127.0.0.1:3000";
#[component]
pub fn User() -> impl IntoView {
    view! {{
        let get_info = move || OnlyCookie::new();
        let results = create_resource(get_info,
            move |get: OnlyCookie| async move {
                match get.user_info(URL).await {
                    Ok(res) => Some(res),
                    Err(_) => None
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
    }}
}