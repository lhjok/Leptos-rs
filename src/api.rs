use serde::{ Deserialize, Serialize };
use gloo_net::http::{ Request, Response };
use serde::de::DeserializeOwned;

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

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub message: String
}

impl AdminLogin {
    pub async fn login(&self, path: & str) -> Result<AdminLoginRes, Error> {
        let url = format!("{}/admin/login", path);
        let response = Request::post(&url).json(self)?.send().await?;
        into_json(response).await
    }
}

async fn into_json<T>(response: Response) -> Result<T, Error>
where T: DeserializeOwned {
    if response.ok() {
        Ok(response.json().await?)
    } else {
        Err(response.json::<Error>().await?.into())
    }
}