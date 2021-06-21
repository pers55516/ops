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
#[cfg(feature = "hyper_server")]
mod server;
mod status;
#[cfg(feature = "trillium_server")]
mod trillium;

pub use crate::check::NamedChecker;
pub use crate::error::Error;
#[cfg(feature = "hyper_server")]
pub use crate::server::server;
pub use crate::status::{StatusBuilder, StatusNoChecks, StatusWithChecks};
#[cfg(feature = "trillium_server")]
pub use crate::trillium::router;
pub use ops_core::{async_trait, CheckResponse, Checker};

/// Result type often returned from methods that can have ops `Error`s.
pub type Result<T> = ::std::result::Result<T, Error>;
