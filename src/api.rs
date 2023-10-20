use gloo_net::Error;
use serde::{ Deserialize, Serialize };
use gloo_net::http::Request;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UniteRes {
    pub status: String,
    pub success: String,
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
    pub data: AdminInfo
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
    pub data: UserInfo
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Singout {
    pub status: String,
    pub success: String
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct UserToken {
    pub token: String,
    pub types: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthBody {
    pub data: String,
    pub status: String,
    pub message: String
}

impl User {
    pub async fn login(&self, path: &str, name: &str) -> Result<AuthBody, Error> {
        let url = format!("{}/{}/login", path, name);
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
            avatar: String::from("static/admin.png"),
            status: 1
        }
    }
}

impl AdminInfo {
    pub async fn signin(&self, path: &str) -> Result<UniteRes, Error> {
        let url = format!("{}/admin/signin", path);
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
            avatar: String::from("static/user.png"),
            status: 1
        }
    }
}

impl UserInfo {
    pub async fn signin(&self, path: &str) -> Result<UniteRes, Error> {
        let url = format!("{}/user/signin", path);
        let response = Request::post(&url).json(self)?.send().await?;
        response.json().await
    }
}

pub struct AuthRuest {
    pub token: String,
    pub types: String
}

impl AuthRuest {
    pub async fn admin_info(self, path: &str) -> Result<AdminInfoRes, Error> {
        let url = format!("{}/admin/info", path);
        let response = Request::get(&url)
            .header("Authorization", &format!("{} {}", self.types, self.token))
            .send().await?;
        response.json().await
    }
    pub async fn user_info(self, path: &str) -> Result<UserInfoRes, Error> {
        let url = format!("{}/user/info", path);
        let response = Request::get(&url)
            .header("Authorization", &format!("{} {}", self.types, self.token))
            .send().await?;
        response.json().await
    }
    pub async fn singout(self, path: &str, name: &str) -> Result<Singout, Error> {
        let url = format!("{}/{}/singout", path, name);
        let response = Request::get(&url)
            .header("Authorization", &format!("{} {}", self.types, self.token))
            .send().await?;
        response.json().await
    }
}