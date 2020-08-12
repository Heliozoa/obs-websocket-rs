//! OBS should be running with a WebSocket server running on port 4444 and password set to 1234.
//! Try doing various things in OBS and see what events pop up!

use obs_websocket::{futures::stream::StreamExt, Obs};

fn main() {
    env_logger::init();

    let future = async {
        let (mut obs, mut event_receiver) = Obs::connect("localhost", 4444).await.unwrap();
        obs.authenticate("1234").await.unwrap();
        while let Some(event) = event_receiver.next().await {
            println!("{:#?}", event);
        }
    };
    smol::run(future);
}
