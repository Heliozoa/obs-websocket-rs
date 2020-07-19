//! OBS should be running with a WebSocket server running on port 4444 and password set to 1234.
//! Here, a simple GetVersion request is sent to OBS.

use obs_websocket::{requests::GetVersion, Obs};

fn main() {
    env_logger::init();

    let future = async {
        let mut obs = Obs::new();
        let _event_receiver = obs.connect("localhost", 4444).await.unwrap();
        obs.authenticate("1234").await.unwrap();
        let response = obs.request(&GetVersion::default()).await.unwrap();
        println!("{:#?}", response);
    };
    smol::run(future);
}
