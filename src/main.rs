#![allow(non_snake_case)]
mod views;
use leptos::*;
use leptos_router::*;
use views::{ Login, Home, Admin, Error };
use leptonic::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/login" view=Login/>
                    <Route path="/admin" view=Admin/>
                    <Route path="/404" view=Error/>
                </Routes>
            </Router>
        </Root>
    }
}