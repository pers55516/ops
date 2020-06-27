//! # Ops
//!
//! Provides standard endpoints for monitoring the health of your application.
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

mod check;
mod error;
mod health;
mod server;
mod status;

pub use check::NamedChecker;
pub use error::Error;
pub use ops_core::{async_trait, CheckResponse, Checker};
pub use server::server;
pub use status::{StatusBuilder, StatusNoChecks, StatusWithChecks};

/// Result type often returned from methods that can have ops `Error`s.
pub type Result<T> = ::std::result::Result<T, Error>;
