#![allow(non_snake_case)]
mod api;
mod views;
use leptos::*;
use leptos_router::*;
use console_error_panic_hook::set_once;
use views::{ Login, Signup, Home, Error };
use views::pages::admin::content::AdminIndex;
use views::pages::admin::{ Admin, AdminError };
use views::pages::user::content::UserIndex;
use views::pages::user::{ User, UserError };
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
                <main class="h-screen">
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/logins" view=|cx|
                            view! { cx, <Login role="admin"/> }/>
                        <Route path="/login" view=|cx|
                            view! { cx, <Login role="user"/> }/>
                        <Route path="/signup" view=Signup/>
                        <Route path="/user" view=User>
                            <Route path="index" view=UserIndex/>
                            <Route path="/*any" view=UserError/>
                            <Route path="" view=|cx| view! { cx,
                                <Redirect path="/user/index"/> }/>
                        </Route>
                        <Route path="/admin" view=Admin>
                            <Route path="index" view=AdminIndex/>
                            <Route path="/*any" view=AdminError/>
                            <Route path="" view=|cx| view! { cx,
                                <Redirect path="/admin/index"/> }/>
                        </Route>
                        <Route path="/*any" view=Error/>
                    </Routes>
                </main>
            </Router>
        </Root>
    }
}