use std::result;

pub mod config;
mod error;
pub mod home;
pub mod platforms;
pub mod tmux_interactor;

pub use error::Error;
pub type Result<T> = result::Result<T, Error>;
