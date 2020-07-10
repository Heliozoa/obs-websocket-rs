//! Rust API for the obs-websocket plugin

pub mod common_types;
pub mod error;
mod events;
pub mod obs;
pub mod requests;
mod responses;

pub use events::Event;
pub use futures;
pub use obs::Obs;
