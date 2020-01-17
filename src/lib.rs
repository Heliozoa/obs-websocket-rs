use tungstenite::{client::AutoStream, connect, protocol::WebSocket, Message};
use url::Url;

mod events;
mod requests;
mod typedefs;
use requests::Request;

pub struct Observe {
    socket: Option<WebSocket<AutoStream>>,
}

impl Observe {
    pub fn new() -> Self {
        Observe { socket: None }
    }

    pub fn connect(&mut self) {
        let (socket, response) =
            connect(Url::parse("ws://localhost:4444").unwrap()).expect("Can't connect");
        self.socket = Some(socket);
    }

    pub fn get_version(&mut self) {
        let socket = self.socket.as_mut().unwrap();
        let req = Request::new();
        let json = serde_json::to_string(&req).unwrap();
        socket.write_message(Message::Text(json)).unwrap();
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut o = Observe::new();
        o.connect();
        o.get_version();
        unimplemented!();
    }
}
