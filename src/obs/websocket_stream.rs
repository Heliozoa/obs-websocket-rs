//! Wrapper for WebSocket that implements Stream

use super::Message;

use futures::{
    task::{Context, Poll},
    Stream,
};
use log::{error, warn};
use std::{io::ErrorKind as IoError, net::TcpStream, pin::Pin};
use thiserror::Error;
use tungstenite::{Error as WebSocketError, Message as WebSocketMessage, WebSocket};

pub(super) struct WebSocketStream(pub WebSocket<TcpStream>);

// currently (probably) does not handle errors properly
impl Stream for WebSocketStream {
    type Item = Result<Message, StreamError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let message = Pin::into_inner(self).0.read_message();
        match message {
            // connection closed, close stream with error so handler can react
            Ok(WebSocketMessage::Close(_)) => Poll::Ready(Some(Err(StreamError::Close))),
            // OK, return message
            Ok(message) => Poll::Ready(Some(Ok(Message::Incoming(message)))),
            Err(error) => match error {
                WebSocketError::Io(error) => match error.kind() {
                    // would block, wait
                    IoError::WouldBlock => {
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    // other IO error, end stream
                    kind => {
                        warn!("other IO error: {:#?}", kind);
                        Poll::Ready(Some(Err(StreamError::Io(error))))
                    }
                },
                // something went wrong
                WebSocketError::Protocol(protocol_violation) => {
                    error!("protocol violation: {:#?}", protocol_violation);
                    Poll::Ready(Some(Err(StreamError::Protocol(
                        protocol_violation.to_string(),
                    ))))
                }
                // tungstenite error, end stream
                error => Poll::Ready(Some(Err(StreamError::Tungstenite(error)))),
            },
        }
    }
}

#[derive(Error, Debug)]
pub enum StreamError {
    #[error("Close handler")]
    Close,

    #[error(transparent)]
    Tungstenite(#[from] tungstenite::error::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("WS protocol violation: {0}")]
    Protocol(String),
}
