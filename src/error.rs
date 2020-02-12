use serde_json::error::Error as JsonError;
use std::io::Error as IoError;
use tungstenite::{
    error::Error as TungsteniteError,
    handshake::{HandshakeError, HandshakeRole},
};

#[derive(Debug)]
pub enum Error {
    Custom(String),
    ObsError(String),
    HandshakeInterrupted,
    HandshakeFailed(TungsteniteError),
    Tungstenite(TungsteniteError),
    Json(JsonError),
    Io(IoError),
}

impl From<TungsteniteError> for Error {
    fn from(err: TungsteniteError) -> Error {
        Error::Tungstenite(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}

impl<T: HandshakeRole> From<HandshakeError<T>> for Error {
    fn from(err: HandshakeError<T>) -> Error {
        match err {
            HandshakeError::Failure(err) => Error::HandshakeFailed(err),
            HandshakeError::Interrupted(_) => Error::HandshakeInterrupted,
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}
