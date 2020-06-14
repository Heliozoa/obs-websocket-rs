// requires an OBS WebSocket server to be on with server port 4444 and password 1234

use obs_websocket::{obs::Obs, requests};

const ADDRESS: &'static str = "localhost";
const PORT: u16 = 4444;
const PASSWORD: &'static str = "1234";

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn asyncer() {
    init();

    smol::run(async {
        let mut obs = Obs::new();
        obs.connect(ADDRESS, PORT).await.unwrap();
        obs.authenticate(PASSWORD).await.unwrap();
        let req = requests::GetVersion::default();
        let res = obs.request(&req).await.unwrap();
        println!("{:?}", res);
        obs.disconnect().await;
        panic!();
    });
}
