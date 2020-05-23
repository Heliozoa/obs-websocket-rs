//! Wrapper for WebSocket that implements Stream

use super::Message;

use futures::{
    task::{Context, Poll},
    Stream,
};
use log::{error, warn};
use std::{io::ErrorKind as IoError, net::TcpStream, pin::Pin};
use tungstenite::{Error as WebSocketError, WebSocket};

pub(super) struct WebSocketStream(pub WebSocket<TcpStream>);

impl Stream for WebSocketStream {
    type Item = Result<Message, ()>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let message = Pin::into_inner(self).0.read_message();
        match message {
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
                    other_error => {
                        warn!("other IO error: {:#?}", other_error);
                        Poll::Ready(None) // TODO: handle properly
                    }
                },
                // something went wrong
                WebSocketError::Protocol(protocol_violation) => {
                    error!("protocol violation: {:#?}", protocol_violation);
                    Poll::Ready(Some(Err(()))) // TODO: handle properly
                }
                // other error, end stream
                other_error => {
                    warn!("other error: {:#?}", other_error);
                    Poll::Ready(None) // TODO: handle properly
                }
            },
        }
    }
}
