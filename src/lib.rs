use base64;
use serde::de::DeserializeOwned;
use serde_json::Value;
use sha2::{Digest, Sha256};
use tungstenite::{client::AutoStream, connect, protocol::WebSocket, Message};
use url::Url;

mod events;
mod requests;
mod typedefs;

type Result<T> = std::result::Result<T, String>;

pub struct Obs {
    socket: Option<WebSocket<AutoStream>>,
}

impl Obs {
    pub fn new() -> Self {
        Obs { socket: None }
    }

    pub fn connect(&mut self) {
        let (socket, _response) =
            connect(Url::parse("ws://localhost:4444").unwrap()).expect("Can't connect");
        self.socket = Some(socket);
    }

    fn get<T: DeserializeOwned>(&mut self, json: Value) -> Result<T> {
        let socket = self.socket.as_mut().unwrap();
        let json = json.to_string();
        println!("SENT: {}", json);
        socket.write_message(Message::Text(json)).unwrap();
        let response = socket
            .read_message()
            .expect("Error reading message")
            .to_string();
        println!("RECV: {}", response);
        let parsed: requests::Response = serde_json::from_str(&response).unwrap();
        if let requests::Status::Ok = parsed.status {
            Ok(serde_json::from_str(&response).unwrap())
        } else {
            Err(parsed.error.unwrap())
        }
    }

    pub fn get_version(&mut self) -> Result<requests::GetVersion> {
        self.get(requests::get_version("0"))
    }

    pub fn get_auth_required(&mut self) -> Result<requests::GetAuthRequired> {
        self.get(requests::get_auth_required("0"))
    }

    pub fn authenticate(&mut self) -> Result<requests::Response> {
        let auth: requests::GetAuthRequired = self.get(requests::get_auth_required("0")).unwrap();
        if auth.auth_required {
            let password = "todo";
            let challenge = auth.challenge.unwrap();
            let salt = auth.salt.unwrap();

            let secret_string = format!("{}{}", password, salt);
            let secret_hash = Sha256::digest(secret_string.as_bytes());
            let secret = base64::encode(&secret_hash);

            let auth_response_string = format!("{}{}", secret, challenge);
            let auth_response_hash = Sha256::digest(auth_response_string.as_bytes());
            let auth_response = base64::encode(&auth_response_hash);
            Ok(self
                .get(requests::authenticate("0", &auth_response))
                .unwrap())
        } else {
            Err("no auth required".to_string())
        }
    }

    pub fn list_profiles(&mut self) -> Result<requests::ListProfiles> {
        self.get(requests::list_profiles("0"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_version() {
        let mut o = Obs::new();
        o.connect();
        o.get_version().unwrap();
    }

    #[test]
    fn get_auth_required() {
        let mut o = Obs::new();
        o.connect();
        o.get_auth_required().unwrap();
    }

    #[test]
    fn authenticate() {
        let mut o = Obs::new();
        o.connect();
        o.authenticate().unwrap();
    }

    #[test]
    fn list_profiles() {
        let mut o = Obs::new();
        o.connect();
        let res = o.list_profiles().unwrap();
        println!("{:?}", res);
        unimplemented!()
    }
}
