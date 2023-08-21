#![allow(non_snake_case)]
mod views;
use leptos::*;
use leptos_router::*;
use views::{ Login, Signup, Home, Admin, Error };
use views::pages::content::AdminIndex;
use views::pages::AdminError;
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