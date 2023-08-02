pub mod auth;
pub mod client;
mod error;
pub mod model;

pub type Result<T> = std::result::Result<T, error::Error>;