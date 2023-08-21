#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;
use super::pages::{
    Aside, Footer, Header, Content
};

#[component]
pub fn Admin(cx: Scope) -> impl IntoView {
    view! { cx,
        <Header/>
        <Aside/>
        <Content>
            <Outlet/>
        </Content>
        <Footer/>
    }
}
