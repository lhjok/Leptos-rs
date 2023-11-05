#![allow(non_snake_case)]
use leptos::*;
use leptos::html::Input;
use leptos::logging::log;
use web_sys::FormData;
use leptonic::prelude::*;
use crate::api::{
    UploadImg, NormRes,
    AdminInfoRes
};

#[component]
pub fn AdminConfig() -> impl IntoView {
    let file_input = create_node_ref::<Input>();
    let (data, set_data) = create_signal(None::<UploadImg>);
    let (_result, set_result) = create_signal(None::<NormRes>);
    let info = use_context::<ReadSignal<Option<AdminInfoRes>>>().unwrap();
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
                <label for="formFileSm" class="mb-3 inline-block cursor-pointer \
                    text-neutral-700 dark:text-neutral-200">
                    <img src=move || info.get().unwrap().data.avatar
                        class="rounded-full border-4 border-neutral-350 mb-0.5"
                        style="height: 65px; width: 65px" alt="" loading="lazy"/>
                    "更换头像"
                </label>
                <input class="relative mb-4 block w-64 min-w-0 flex-auto cursor-pointer rounded \
                border-solid border-neutral-300 bg-clip-padding px-3 py-[0.32rem] text-xs font-normal \
                text-neutral-700 transition duration-300 ease-in-out file:-mx-3 file:-my-[0.32rem] \
                file:cursor-pointer file:overflow-hidden file:rounded-none file:border-0 file:border-solid \
                file:border-inherit file:bg-neutral-100 file:px-3 file:py-[0.32rem] file:text-neutral-700 \
                file:transition file:duration-150 file:ease-in-out file:[border-inline-end-width:1px] \
                file:[margin-inline-end:0.75rem] hover:file:bg-neutral-200 focus:border-primary \
                focus:text-neutral-700 focus:shadow-te-primary focus:outline-none dark:border-neutral-600 \
                border dark:text-neutral-200 dark:file:bg-neutral-700 dark:file:text-neutral-100 \
                dark:focus:border-primary" id="formFileSm" type="file" accept="image/*"
                    on:change=file_load node_ref=file_input multiple=false />
                <button class="inline-block rounded bg-primary pl-4 pr-6 pb-2 pt-2 text-sm mb-8 \
                font-medium uppercase leading-normal text-white shadow-[0_4px_9px_-4px_#3b71ca] transition \
                duration-150 ease-in-out hover:bg-primary-600 hover:shadow-[0_8px_9px_-4px_rgba\
                (59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)] focus:bg-primary-600 focus:shadow-\
                [0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)] focus:outline-none \
                focus:ring-0 active:bg-primary-700 active:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),\
                0_4px_18px_0_rgba(59,113,202,0.2)] dark:shadow-[0_4px_9px_-4px_rgba(59,113,202,0.5)] \
                dark:hover:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.2),0_4px_18px_0_rgba(59,113,202,0.1)] \
                dark:focus:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.2),0_4px_18px_0_rgba(59,113,202,0.1)] \
                dark:active:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.2),0_4px_\
                18px_0_rgba(59,113,202,0.1)]" on:click=move |_| click_upload() >
                    <svg class="w-[25px] h-[15px] fill-[#ffffff] inline-block mb-0.5 mr-0.5"
                        viewBox="0 0 576 512" xmlns="http://www.w3.org/2000/svg">
                        <path d="M144 480C64.5 480 0 415.5 0 336c0-62.8 40.2-116.2 96.2-135.9c-.1-2.7-.\
                        2-5.4-.2-8.1c0-88.4 71.6-160 160-160c59.3 0 111 32.2 138.7 80.2C409.9 \
                        102 428.3 96 448 96c53 0 96 43 96 96c0 12.2-2.3 23.8-6.4 34.6C596 238.4 640 \
                        290.1 640 352c0 70.7-57.3 128-128 128H144zm79-217c-9.4 9.4-9.4 24.6 0 \
                        33.9s24.6 9.4 33.9 0l39-39V392c0 13.3 10.7 24 24 24s24-10.7 \
                        24-24V257.9l39 39c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-80-80c-9.4-9.4-\
                        24.6-9.4-33.9 0l-80 80z"></path>
                    </svg>"上传"
                </button>
            </form>
            <Separator />
        </div>
    }
}