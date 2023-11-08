#![allow(non_snake_case)]
use leptos::*;
use leptos::logging::log;
use web_sys::FormData;
use leptonic::prelude::*;
use crate::views::FileInput;
use crate::api::{
    UploadImg, NormRes,
    AdminInfoRes
};

#[component]
pub fn AdminConfig() -> impl IntoView {
    let (data, set_data) = create_signal(None::<FormData>);
    let (_upload, set_upload) = create_signal(None::<NormRes>);
    let info = use_context::<ReadSignal<Option<AdminInfoRes>>>().unwrap();
    // 上传文件到服务器
    let action = create_action(move |data: &FormData| {
        let file = UploadImg::from(data.clone());
        async move {
            if let Ok(res) = file.upload("admin").await {
                set_upload.set(Some(res))
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
                <label for="formFileImg" class="mb-3 inline-block cursor-pointer \
                    text-neutral-700 dark:text-neutral-200">
                    <img src=move || info.get().unwrap().data.avatar
                        class="rounded-full border-4 border-neutral-350 mb-0.5"
                        style="height: 65px; width: 65px" alt="" loading="lazy"/>
                    "更换头像"
                </label>
                <FileInput id="formFileImg" accept="image/*" form_data=set_data multiple=false />
                <button class="btn-upload" on:click=move |_| click_upload() >
                    <svg class="w-[25px] h-[15px] fill-[#ffffff] inline-block mb-0.5 mr-0.5"
                        viewBox="0 0 576 512" xmlns="http://www.w3.org/2000/svg">
                        <path d="M144 480C64.5 480 0 415.5 0 336c0-62.8 40.2-116.2 \
                        96.2-135.9c-.1-2.7-.2-5.4-.2-8.1c0-88.4 71.6-160 160-160c59.3 0 \
                        111 32.2 138.7 80.2C409.9 102 428.3 96 448 96c53 0 96 43 96 96c0 \
                        12.2-2.3 23.8-6.4 34.6C596 238.4 640 290.1 640 352c0 70.7-57.3 \
                        128-128 128H144zm79-217c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 \
                        0l39-39V392c0 13.3 10.7 24 24 24s24-10.7 24-24V257.9l39 39c9.4 \
                        9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-80-80c-9.4-9.4-24.\
                        6-9.4-33.9 0l-80 80z"></path>
                    </svg>"上传"
                </button>
            </form>
            <Separator/>
        </div>
    }
}