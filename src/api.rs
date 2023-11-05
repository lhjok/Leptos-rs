use gloo_net::Error;
use web_sys::FormData;
use serde::{ Deserialize, Serialize };
use gloo_net::http::Request;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NormRes {
    pub status: String,
    pub data: String,
    pub message: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AdminInfo {
    #[serde(skip_serializing)]
    pub id: i32,
    pub mail: String,
    pub username: String,
    pub password: String,
    pub phone: String,
    pub avatar: String,
    pub status: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AdminInfoRes {
    pub status: String,
    pub data: AdminInfo,
    pub message: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(skip_serializing)]
    pub id: i32,
    pub mail: String,
    pub username: String,
    pub password: String,
    pub phone: String,
    pub avatar: String,
    pub status: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserInfoRes {
    pub status: String,
    pub data: UserInfo,
    pub message: String
}

// 服务器请求地址
const URL: &'static str = "https://127.0.0.1:3000";

impl User {
    pub async fn login(&self, name: &str) -> Result<NormRes, Error> {
        let url = format!("{}/{}/login", URL, name);
        let response = Request::post(&url).json(self)?.send().await?;
        response.json().await
    }
}

impl Default for AdminInfo {
    fn default() -> AdminInfo {
        AdminInfo {
            id: 0,
            mail: String::from(""),
            username: String::from(""),
            password: String::from(""),
            phone: String::from(""),
            avatar: String::from("/static/images/admin/admin.png"),
            status: 1
        }
    }
}

impl AdminInfo {
    pub async fn signin(&self) -> Result<NormRes, Error> {
        let url = format!("{}/admin/signin", URL);
        let response = Request::post(&url).json(self)?.send().await?;
        response.json().await
    }
}

impl Default for UserInfo {
    fn default() -> UserInfo {
        UserInfo {
            id: 0,
            mail: String::from(""),
            username: String::from(""),
            password: String::from(""),
            phone: String::from(""),
            avatar: String::from("/static/images/user/user.png"),
            status: 1
        }
    }
}

impl UserInfo {
    pub async fn signin(&self) -> Result<NormRes, Error> {
        let url = format!("{}/user/signin", URL);
        let response = Request::post(&url).json(self)?.send().await?;
        response.json().await
    }
}

#[derive(Clone)]
pub struct UploadImg {
    pub data: FormData
}

impl UploadImg {
    pub fn from(data: FormData) -> Self {
        UploadImg{data}
    }
    pub async fn upload(&self, name: &str) -> Result<NormRes, Error> {
        let url = format!("{}/{}/upload", URL, name);
        let response = Request::post(&url)
            .body(&self.data)?.send().await?;
        response.json().await
    }
}

#[derive(Clone, PartialEq)]
pub struct OnlyCookie;

impl OnlyCookie {
    pub fn new() -> Self { OnlyCookie{} }
    pub async fn admin_info(self) -> Result<AdminInfoRes, Error> {
        let url = format!("{}/admin/info", URL);
        let response = Request::get(&url).send().await?;
        response.json().await
    }
    pub async fn user_info(self) -> Result<UserInfoRes, Error> {
        let url = format!("{}/user/info", URL);
        let response = Request::get(&url).send().await?;
        response.json().await
    }
    pub async fn singout(self, name: &str) -> Result<NormRes, Error> {
        let url = format!("{}/{}/singout", URL, name);
        let response = Request::get(&url).send().await?;
        response.json().await
    }
}