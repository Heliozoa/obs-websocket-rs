//! Rust API for the obs-websocket plugin

pub mod events;

pub mod requests;
pub mod responses;

mod common_types;
mod error;
mod obs;

pub use error::ObsError;
pub use futures;
pub use obs::Obs;
