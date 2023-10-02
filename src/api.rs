use gloo_net::Error;
use serde::{ Deserialize, Serialize };
use gloo_net::http::Request;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginRes {
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
pub struct Singout {
    pub status:  String,
    pub success: String
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct UserName {
    pub username:  String
}

impl User {
    pub async fn login(&self, path: &str, name: &str) -> Result<LoginRes, Error> {
        let url = format!("{}/{}/login", path, name);
        let response = Request::post(&url).json(self)?.send().await?;
        response.json().await
    }
}

pub struct GetQuery {
    pub user: Vec<(&'static str, String)>
}

impl GetQuery {
    pub async fn admin_info(self, path: &str) -> Result<AdminInfoRes, Error> {
        let url = format!("{}/admin/info", path);
        let response = Request::get(&url).query(self.user).send().await?;
        response.json().await
    }
    pub async fn signup(self, path: &str, name: &str) -> Result<Singout, Error> {
        let url = format!("{}/{}/singout", path, name);
        let response = Request::get(&url).query(self.user).send().await?;
        response.json().await
    }
}