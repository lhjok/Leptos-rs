#![allow(non_snake_case)]
use leptos::*;
use leptos::html::Input;
use web_sys::FormData;

#[component]
pub fn FileInput(
    id: &'static str,
    accept: &'static str,
    form_data: WriteSignal<Option<FormData>>,
    multiple: bool
) -> impl IntoView {
    let file_input = create_node_ref::<Input>();
    let file_load = move |_| {
        if let Some(files) = file_input.get().and_then(|f|f.files()) {
            let file = files.get(0).unwrap();
            let data = FormData::new().unwrap();
            data.append_with_blob_and_filename(&file.name(), &file, &file.name()).unwrap();
            form_data.update(|d| *d = Some(data));
        }
    };
    view! {
        <input class="file-input" id=id type="file" accept=accept on:change=file_load
            node_ref=file_input multiple=multiple />
    }
}