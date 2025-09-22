#![warn(clippy::pedantic)]
#![allow(dead_code)] // while developing

mod error;
mod init;
mod pod;

pub use error::Error;
pub use pod::{Command, Pod};

pub type Result<T> = std::result::Result<T, Error>;
