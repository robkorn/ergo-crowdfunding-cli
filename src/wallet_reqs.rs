
use crate::campaign::Campaign;
use handlebars::Handlebars;
use reqwest;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde::Deserialize;

static SEND_PAYMENT_TEMPLATE : &'static str = r#"[{"address":"{{address}}","value":{{value}} }]"#;
    
#[derive(Deserialize)]
struct P2SAddress {
    address: String
}

/// Gets a list of all addresses from the local unlocked node wallet.
pub fn get_wallet_addresses(api_key: &String) -> Vec<String> {
    let endpoint = "http://0.0.0.0:9052/wallet/addresses";
    let client = reqwest::Client::new();
    let hapi_key = HeaderValue::from_str(&api_key).expect("Failed to create header value from api key.");
    let mut res = client.get(endpoint)
                .header("accept", "application/json")
                .header("api_key", hapi_key)
                .header(CONTENT_TYPE, "application/json")
                .send()
                .expect("Failed to send request to local node. Please make sure it is running on API port 9052.");


    let mut addresses : Vec<String> = vec![];
    for segment in res.text().expect("Failed to get addresses from wallet.").split("\""){
        println!("{}", segment);
        let seg = segment.trim();
        if seg.chars().next().unwrap() == '9' {
           addresses.push(seg.to_string()); 
        }
    }
    if addresses.len() == 0 {
        panic!("No addresses were found. Please make sure it is running on API port 9052 and your wallet is unlocked.");
    }
    println!("{:?}", addresses);
    addresses
}

/// Get P2S Address for Backer to submit to for the Campaign
pub fn get_p2s_address(api_key: &String, campaign: &Campaign,  backer_pub_key: &String) -> String {
    let endpoint = "http://0.0.0.0:9052/script/p2sAddress";
    let client = reqwest::Client::new();
    let hapi_key = HeaderValue::from_str(&api_key).expect("Failed to create header value from api key.");
    let mut res = client.post(endpoint)
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

/// Send payment from unlocked wallet to given address via local node api
pub fn send_wallet_payment(api_key: &String, address: &String, amount: u32) {
    let nanoerg_amount = amount * 1000000000;
    let json_body = json!({ "address": address,
                            "value": nanoerg_amount });
    let reg = Handlebars::new();

    if let Ok(pb) = reg.render_template(SEND_PAYMENT_TEMPLATE, &json_body){
        let endpoint = "http://0.0.0.0:9052/wallet/payment/send";
        let client = reqwest::Client::new();
        let hapi_key = HeaderValue::from_str(&api_key).expect("Failed to create header value from api key.");
        let res = client.post(endpoint)
                  .header("accept", "application/json")
                  .header("api_key", hapi_key)
                  .header(CONTENT_TYPE, "application/json")
                  .body(pb)
                  .send();

        if let Ok(mut r) = res {
            if let Ok(text_response) = r.text(){
                println!("Response from the wallet: {}", text_response);
            }

        }
        else if let Err(e) = res {
            println!("Error: {:?}", e);
            panic!("Failed to send wallet payment.");
        }

    }




}