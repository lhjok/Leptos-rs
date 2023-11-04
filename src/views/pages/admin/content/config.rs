#![allow(non_snake_case)]
use leptos::*;
use leptos::html::Input;
use leptos::logging::log;
use web_sys::FormData;
use crate::api::{ UploadImg, NormRes };

#[component]
pub fn AdminConfig() -> impl IntoView {
    let file_input = create_node_ref::<Input>();
    let (data, set_data) = create_signal(None::<UploadImg>);
    let (_result, set_result) = create_signal(None::<NormRes>);
    // 浏览器选择文件事件函数
    let file_load = move |_| {
        if let Some(files) = file_input.get().and_then(|f|f.files()) {
            let file = files.get(0).unwrap();
            let form_data = FormData::new().unwrap();
            form_data.append_with_blob_and_filename(&file.name(), &file, &file.name()).unwrap();
            set_data.set(Some(UploadImg::from(form_data)));
        }
    };
    // 上传文件到服务器
    let action = create_action(move |data: &UploadImg| {
        let file = data.clone();
        async move {
            if let Ok(res) = file.upload("admin").await {
                set_result.set(Some(res))
            }
        }
    });
    // 点击调用上传函数
    let click_upload = move || {
        match data.get() {
            Some(data) => action.dispatch(data),
            None => log!("没有读取到文件")
        }
    };
    view! {
        <div class="h-full p-10">
            <h3>系统设置</h3>
            <form on:submit=|event| event.prevent_default()>
                <label for="formFileSm" class="mb-3 inline-block cursor-pointer text-neutral-700 dark:text-neutral-200">
                    <img src="/static/images/admin/admin-IQX2kk-avatar.jpg" class="rounded-full"
                        style="height: 60px; width: 60px" alt="" loading="lazy"/>
                    "更换头像"
                </label>
                <input class="relative mb-4 block w-64 min-w-0 flex-auto cursor-pointer rounded border \
                border-solid border-neutral-300 bg-clip-padding px-3 py-[0.32rem] text-xs font-normal \
                text-neutral-700 transition duration-300 ease-in-out file:-mx-3 file:-my-[0.32rem] \
                file:cursor-pointer file:overflow-hidden file:rounded-none file:border-0 file:border-solid \
                file:border-inherit file:bg-neutral-100 file:px-3 file:py-[0.32rem] file:text-neutral-700 \
                file:transition file:duration-150 file:ease-in-out file:[border-inline-end-width:1px] \
                file:[margin-inline-end:0.75rem] hover:file:bg-neutral-200 focus:border-primary \
                focus:text-neutral-700 focus:shadow-te-primary focus:outline-none dark:border-neutral-600 \
                dark:text-neutral-200 dark:file:bg-neutral-700 dark:file:text-neutral-100 \
                dark:focus:border-primary" id="formFileSm" type="file" accept="image/*"
                    on:change=file_load node_ref=file_input multiple=false />
                <button class="inline-block rounded bg-primary px-6 pb-2 pt-2.5 text-xs \
                font-medium uppercase leading-normal text-white shadow-[0_4px_9px_-4px_#3b71ca] transition \
                duration-150 ease-in-out hover:bg-primary-600 hover:shadow-[0_8px_9px_-4px_rgba\
                (59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)] focus:bg-primary-600 focus:shadow-\
                [0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)] focus:outline-none \
                focus:ring-0 active:bg-primary-700 active:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),\
                0_4px_18px_0_rgba(59,113,202,0.2)] dark:shadow-[0_4px_9px_-4px_rgba(59,113,202,0.5)] \
                dark:hover:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.2),0_4px_18px_0_rgba(59,113,202,0.1)] \
                dark:focus:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.2),0_4px_18px_0_rgba(59,113,202,0.1)] \
                dark:active:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.2),0_4px_\
                18px_0_rgba(59,113,202,0.1)]" on:click=move |_| click_upload() >提交</button>
            </form>
        </div>
    }
}