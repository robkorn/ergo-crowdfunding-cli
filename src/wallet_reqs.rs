
use crate::campaign::{Campaign, BackingTx};
use handlebars::Handlebars;
use reqwest;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde::Deserialize;

static SEND_PAYMENT_TEMPLATE : &'static str = r#"[{"address":"{{address}}","value":{{value}} }]"#;

#[derive(Deserialize)]
struct P2SAddress {
    address: String
}

/// Gets list of addresses and asks the user to select one
pub fn select_wallet_address(api_key: &String) -> String {
    let address_list = get_wallet_addresses(api_key);
    if address_list.len() == 1 {
        return address_list[0].clone();
    }

    let mut n = 0;
    for address in &address_list {
        n += 1;
        println!("{}. {}", n, address);
    }
    println!("Which address would you like to select?");
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input){
        if let Ok(input_n) = input.trim().parse::<usize>(){
            if input_n > address_list.len() || input_n < 1 {
                println!("Please select an address within the range.");
                return select_wallet_address(api_key);
            }
            return address_list[input_n-1].clone();

        }
    }
    return select_wallet_address(api_key);
}

/// Gets a list of all addresses from the local unlocked node wallet
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
        let seg = segment.trim();
        if seg.chars().next().unwrap() == '9' {
           addresses.push(seg.to_string()); 
        }
    }
    if addresses.len() == 0 {
        panic!("No addresses were found. Please make sure your node running on API port 9052 and your wallet is unlocked.");
    }
    addresses
}

/// Get P2S Address for Backer to submit to for the Campaign
pub fn get_p2s_address(api_key: &String, campaign: &Campaign,  backer_address: &String) -> String {
    let endpoint = "http://0.0.0.0:9052/script/p2sAddress";
    let client = reqwest::Client::new();
    let hapi_key = HeaderValue::from_str(&api_key).expect("Failed to create header value from api key.");
    let mut res = client.post(endpoint)
                .header("accept", "application/json")
                .header("api_key", hapi_key)
                .header(CONTENT_TYPE, "application/json")
                .body(campaign.build_script(backer_address))
                .send()
                .expect("Failed to send request to local node. Please make sure it is running on API port 9052.");

    if let Ok(p2saddress) = res.json::<P2SAddress>() {
        return p2saddress.address;
    }
    panic!("Failed to acquire P2S Address. Make sure your node is running and that the data you provided is valid.");
}

/// Send payment from unlocked wallet to given address via local node api. Returns the box identifier.
pub fn send_wallet_payment(api_key: &String, address: &String, amount: u32) -> Option<BackingTx> {
    // let nanoerg_amount = amount * 1000000000;
    // lowered for now for testing purposes
    let nanoerg_amount : u32 = amount * 100000;
    let json_body = json!({ "address": address,
                            "value": nanoerg_amount });
    let reg = Handlebars::new();

    let pb = reg.render_template(SEND_PAYMENT_TEMPLATE, &json_body).ok()?;
    let endpoint = "http://0.0.0.0:9052/wallet/payment/send";
    let client = reqwest::Client::new();
    let hapi_key = HeaderValue::from_str(&api_key).expect("Failed to create header value from api key.");
    let res = client.post(endpoint)
                .header("accept", "application/json")
                .header("api_key", hapi_key)
                .header(CONTENT_TYPE, "application/json")
                .body(pb)
                .send();

    let mut tx_id = res.ok()?.text().ok()?;
    tx_id.retain(|c| c != '"');

    return Some(BackingTx::new(tx_id, amount));
}