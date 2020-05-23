//! Crate error types

use futures::channel::oneshot::Canceled;
use thiserror::Error;
use tungstenite::handshake::{HandshakeError, HandshakeRole};

/// Wraps all the errors that can occur in the crate
#[derive(Debug, Error)]
pub enum ObsError {
    #[error("Connection interrupted")]
    ConnectionInterrupted,
    #[error("Oneshot channel sender closed: {0}")]
    OneshotCanceled(Canceled),
    #[error("Not connected")]
    NotConnected,
    #[error("No authentication required")]
    NoAuthRequired,

    #[error("Error from OBS: {0}")]
    ObsError(String),
    #[error("Handshake interrupted")]
    HandshakeInterrupted,
    #[error("Handshake failed")]
    HandshakeFailed(tungstenite::error::Error),
    #[error("Tungstenite error: {0}")]
    Tungstenite(#[from] tungstenite::error::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::error::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl<T: HandshakeRole> From<HandshakeError<T>> for ObsError {
    fn from(err: HandshakeError<T>) -> ObsError {
        match err {
            HandshakeError::Failure(err) => ObsError::HandshakeFailed(err),
            HandshakeError::Interrupted(_) => ObsError::HandshakeInterrupted,
        }
    }
}
