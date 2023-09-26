pub mod pages;
pub mod home;
pub mod login;
pub mod wrong;
pub mod signup;
pub mod admin;
pub mod error;
pub mod loading;

pub use self::home::Home;
pub use self::login::Login;
pub use self::wrong::LoginErr;
pub use self::loading::Loading;
pub use self::signup::Signup;
pub use self::admin::Admin;
pub use self::error::Error;