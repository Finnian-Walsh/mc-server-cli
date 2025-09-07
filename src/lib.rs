pub mod config;
mod error;
pub mod home;
pub mod platforms;
mod reqwest_client;
pub mod zellij;

pub use error::{Error, Result};
