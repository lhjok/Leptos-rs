pub mod pages;
pub mod home;
pub mod login;
pub mod signup;
pub mod error;
pub mod loading;
pub mod upload;

pub use self::home::Home;
pub use self::login::Login;
pub use self::loading::Loading;
pub use self::signup::Signup;
pub use self::error::Error;
pub use self::upload::FileInput;