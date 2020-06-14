use obs_websocket::Obs;
use obs_websocket::Event;
use obs_websocket::requests::GetVersion;
use obs_websocket::futures::channel::mpsc::UnboundedReceiver;
use obs_websocket::futures::stream::StreamExt;
use smol::Task;

async fn print_events(mut event_receiver: UnboundedReceiver<Event>) {
    println!("awaitign");
    while let Some(event) = event_receiver.next().await {
        println!("event {:?}", event);
    }
    println!("penished");
}

fn main() {
    env_logger::init();

    let future = async {
        let mut obs = Obs::new();
        let mut event_receiver = obs.connect("localhost", 4444).await.unwrap();
        obs.authenticate("todo").await.unwrap();
        let event = event_receiver.next().await;
        println!("{:?}", event);
    };
    smol::run(future);
}
