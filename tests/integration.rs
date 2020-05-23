// requires an OBS WebSocket server to be on with server port 4444 and password 1234

use obs_websocket::{obs::Obs, requests};

const ADDRESS: &'static str = "localhost";
const PORT: u16 = 4444;
const PASSWORD: &'static str = "1234";

#[test]
fn test() {
    let mut obs = Obs::new();
    obs.connect(ADDRESS, PORT).unwrap();
    obs.authenticate(PASSWORD).unwrap();
    let req = requests::GetVersion::default();
    let res = obs.request(&req).unwrap();
    println!("{:?}", res);
    obs.close();
    panic!()
}
