use serde_json::error::Error as JsonError;
use tungstenite::{
    error::Error as TungsteniteError,
    handshake::{HandshakeError, HandshakeRole},
};
use url::ParseError;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    ObsError(String),
    HandshakeInterrupted,
    HandshakeFailed(TungsteniteError),
    Tungstenite(TungsteniteError),
    Json(JsonError),
    Parse(ParseError),
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

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::Parse(err)
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
