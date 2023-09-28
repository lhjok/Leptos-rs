use gloo_net::Error;
use serde::{ Deserialize, Serialize };
use gloo_net::http::Request;

#[derive(Serialize, Deserialize)]
pub struct AdminLogin {
    pub username: String,
    pub password: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AdminLoginRes {
    pub status: String,
    pub success: String,
    pub message: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AdminInfo {
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
    pub status:  String,
    pub data: AdminInfo
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AdminSingout {
    pub status:  String,
    pub success: String
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct UserName {
    pub username:  String
}

impl AdminLogin {
    pub async fn login(&self, path: &str) -> Result<AdminLoginRes, Error> {
        let url = format!("{}/admin/login", path);
        let response = Request::post(&url).json(self)?.send().await?;
        response.json().await
    }
}

pub struct AdminQuery<'a> {
    pub user: Vec<(&'a str, &'a str)>
}

impl AdminQuery<'_> {
    pub async fn info(self, path: &str) -> Result<AdminInfoRes, Error> {
        let url = format!("{}/admin/info", path);
        let response = Request::get(&url).query(self.user).send().await?;
        response.json().await
    }
    pub async fn signup(self, path: &str) -> Result<AdminSingout, Error> {
        let url = format!("{}/admin/singout", path);
        let response = Request::get(&url).query(self.user).send().await?;
        response.json().await
    }
}