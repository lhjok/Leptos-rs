#![allow(non_snake_case)]
mod api;
mod views;
use leptos::*;
use leptos_router::*;
use console_error_panic_hook::set_once;
use views::{ Login, Signup, Home, Error };
use views::pages::admin::content::{ AdminIndex, AdminSignup };
use views::pages::admin::{ Admin, AdminError };
use views::pages::user::content::UserIndex;
use views::pages::user::{ User, UserError };
use leptonic::prelude::*;

fn main() {
    set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <main class="h-screen">
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/logins" view=||
                            view! { <Login role="admin"/> }/>
                        <Route path="/login" view=||
                            view! { <Login role="user"/> }/>
                        <Route path="/signup" view=Signup/>
                        <Route path="/user" view=User>
                            <Route path="index" view=UserIndex/>
                            <Route path="/*any" view=UserError/>
                            <Route path="" view=|| view! {
                                <Redirect path="/user/index"/> }/>
                        </Route>
                        <Route path="/admin" view=Admin>
                            <Route path="index" view=AdminIndex/>
                            <Route path="signup" view=AdminSignup/>
                            <Route path="/*any" view=AdminError/>
                            <Route path="" view=|| view! {
                                <Redirect path="/admin/index"/> }/>
                        </Route>
                        <Route path="/*any" view=Error/>
                    </Routes>
                </main>
            </Router>
        </Root>
    }
}