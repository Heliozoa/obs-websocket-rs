use thiserror::Error;
use tungstenite::handshake::{HandshakeError, HandshakeRole};

#[derive(Debug, Error)]
pub enum ObsError {
    #[error("Connection interrupted")]
    ConnectionInterrupted,
    #[error("Handled channel closed")]
    HandlerChannelClosed,
    #[error("Not connected")]
    NotConnected,

    #[error("OBS error: {0}")]
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
