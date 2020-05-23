//! Event types. Sent by the server as they occur in OBS.

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Event {
    update_type: String,
    stream_timecode: Option<String>,
    rec_timecode: Option<String>,
}
