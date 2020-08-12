//! Crate error types

use async_tungstenite::tungstenite::{
    self,
    handshake::{HandshakeError, HandshakeRole},
};
use futures::channel::oneshot::Canceled;
use std::any::Any;
use thiserror::Error;

/// Wraps all the errors that can occur in the crate
#[derive(Debug, Error)]
pub enum ObsError {
    #[error("Connection interrupted")]
    ConnectionInterrupted,
    #[error("Oneshot channel sender closed")]
    OneshotCanceled(#[source] Canceled),
    #[error("Not connected")]
    NotConnected,
    #[error("No authentication required")]
    NoAuthRequired,
    #[error("Tungstenite timed out")]
    TungsteniteTimeout,
    #[error("Handler thread panicked")]
    HandlerThreadError(Box<dyn Any + Send + 'static>),
    #[error("Error(s) while disconnecting: socket: {socket_error:?}, thread: {thread_error:?}")]
    DisconnectError {
        socket_error: Option<Box<ObsError>>,
        thread_error: Option<Box<ObsError>>,
    },
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
    #[error("Authentication response field \"challenge\" was missing")]
    MissingChallenge,
    #[error("Authentication response field \"salt\" was missing")]
    MissingSalt,
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    #[error("Failed to start thread")]
    Thread(#[source] std::io::Error),
}

impl<T: HandshakeRole> From<HandshakeError<T>> for ObsError {
    fn from(err: HandshakeError<T>) -> ObsError {
        match err {
            HandshakeError::Failure(err) => ObsError::HandshakeFailed(err),
            HandshakeError::Interrupted(_) => ObsError::HandshakeInterrupted,
        }
    }
}

/// Errors that can occur in the handler thread
#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Failed to send response")]
    SendResponse,
    #[error("Tungstenite error")]
    Tungstenite(#[source] tungstenite::Error),
}
