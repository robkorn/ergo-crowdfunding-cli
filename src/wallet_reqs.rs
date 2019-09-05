
use crate::campaign::Campaign;
use reqwest;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde::Deserialize;

#[derive(Deserialize)]
struct P2SAddress {
    address: String
}

pub fn get_p2s_address(campaign: &Campaign, api_key: &String, backer_pub_key: &String) -> String {
    let local_node_url = "http://0.0.0.0:9052/script/p2sAddress";
    let client = reqwest::Client::new();
    let hapi_key = HeaderValue::from_str(&api_key).expect("Failed to create header value from api key.");
    let mut res = client.post(local_node_url)
                .header("accept", "application/json")
                .header("api_key", hapi_key)
                .header(CONTENT_TYPE, "application/json")
                .body(campaign.build_script(backer_pub_key))
                .send()
                .expect("Failed to send request to local node. Please make sure it is running on API port 9052.");

    if let Ok(p2saddress) = res.json::<P2SAddress>() {
        println!("{}", p2saddress.address);
        return p2saddress.address;
    }
    panic!("Failed to acquire P2S Address. Make sure your node is running and that the data you provided is valid.");
}