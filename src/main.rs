#![allow(non_snake_case)]
mod api;
mod views;
use leptos::*;
use leptos_router::*;
use console_error_panic_hook::set_once;
use views::{ Login, Signup, Home, Admin, Error };
use views::pages::content::AdminIndex;
use views::pages::AdminError;
use leptonic::prelude::*;

fn main() {
    set_once();
    mount_to_body(App);
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/login" view=Login/>
                    <Route path="/signup" view=Signup/>
                    <Route path="/admin" view=Admin>
                        <Route path="index" view=AdminIndex/>
                        <Route path="/*any" view=AdminError/>
                        <Route path="" view=|cx| view! { cx,
                            <Redirect path="/admin/index"/> }/>
                    </Route>
                    <Route path="/*any" view=Error/>
                </Routes>
            </Router>
        </Root>
    }
}