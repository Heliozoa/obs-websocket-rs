use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Event {
    update_type: String,
    stream_timecode: Option<String>,
    rec_timecode: Option<String>,
}
