use serde_json::error::Error as JsonError;
use tungstenite::error::Error as TungsteniteError;
use url::ParseError;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    ObsError(String),
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
