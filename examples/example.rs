//! A more "real-world" example for the library.

use obs_websocket::{futures::stream::StreamExt, requests::*, Obs};
use smol::Task;

fn main() {
    env_logger::init();

    smol::run(async {
        let (mut obs, mut event_receiver) = Obs::connect("localhost", 4444).await.unwrap();
        obs.authenticate("1234").await.unwrap();

        // this Task will keep running in the background and print any events that arrive
        Task::spawn(async move {
            while let Some(event) = event_receiver.next().await {
                println!("{:#?}", event);
            }
        })
        .detach();

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        loop {
            buffer.clear();
            println!("press 1 to request with GetVersion");
            println!("press 2 to request with GetStats");
            stdin.read_line(&mut buffer).unwrap();
            match buffer.trim() {
                "1" => {
                    let gv = obs.request(&GetVersion::builder().build()).await.unwrap();
                    println!("version {:#?}", gv);
                }
                "2" => {
                    let gs = obs.request(&GetStats::builder().build()).await.unwrap();

                    println!("stats {:#?}", gs);
                }
                _ => continue,
            };
        }
    });
}
