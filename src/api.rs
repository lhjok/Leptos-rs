use gloo_net::Error;
use serde::{ Deserialize, Serialize };
use gloo_net::http::Request;

#[derive(Serialize, Deserialize)]
pub struct AdminLogin {
    pub username: String,
    pub password: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AdminInfo {
    pub mail: String,
    pub username: String,
    pub password: String,
    pub phone: String,
    pub avatar: String,
    pub status: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AdminName {
    pub username: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AdminLoginRes {
    pub status: String,
    pub success: String,
    pub message: String
}

impl AdminLogin {
    pub async fn login(&self, path: & str) -> Result<AdminLoginRes, Error> {
        let url = format!("{}/admin/login", path);
        let response = Request::post(&url).json(self)?.send().await?;
        response.json().await
    }
}