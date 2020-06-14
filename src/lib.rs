//! Rust API for the obs-websocket plugin

pub mod error;
mod events;
pub mod obs;
pub mod requests;
pub mod responses;

pub use events::Event;
pub use futures;
pub use obs::Obs;
