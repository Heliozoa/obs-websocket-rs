use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Request {
    request_type: String,
    message_id: String,
}

impl Request {
    pub fn new() -> Self {
        Request {
            request_type: "GetVersion".to_string(),
            message_id: "0".to_string(),
        }
    }
}
