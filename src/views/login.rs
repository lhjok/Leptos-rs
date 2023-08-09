#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module=
"/static/scripts/elements.js")]
extern {
    #[wasm_bindgen(js_name= initDropdown)]
    fn init_dropdown(id: &str);
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="login">
            <div class="m-8 relative" data-te-dropdown-ref>
                <button type="button"
                    class="flex btn-secondary"
                    id="dropdownMenuButton1"
                    data-te-dropdown-toggle-ref
                    aria-expanded="false">
                    "Dropdown button"
                    <span class="ml-2 w-2">
                        <svg xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            class="h-5 w-5">
                            <path fill-rule="evenodd"
                                d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 \
                                1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
                                clip-rule="evenodd"/>
                        </svg>
                    </span>
                </button>
                <ul class="ul-menu [&[data-te-dropdown-show]]:block"
                    aria-labelledby="dropdownMenuButton1"
                    data-te-dropdown-menu-ref>
                    <li class="li-menu">
                        <A href="/">"Go Home!"</A>
                    </li>
                    <li class="li-menu">
                        <A href="/admin">"Go Admin!"</A>
                    </li>
                </ul>
            </div>
            { request_animation_frame(move || {
                init_dropdown("[data-te-dropdown-toggle-ref]");
            }) }
        </div>
    }
}
