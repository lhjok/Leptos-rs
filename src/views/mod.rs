pub mod pages;
pub mod home;
pub mod login;
pub mod signup;
pub mod admin;
pub mod error;

pub use self::home::Home;
pub use self::login::Login;
pub use self::signup::Signup;
pub use self::admin::Admin;
pub use self::error::Error;
