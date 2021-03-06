use crate::campaign::{CrowdfundingCampaign, Campaign, BackingTx};
use handlebars::Handlebars;
use reqwest;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use std::io::prelude::*;
use std::fs::{File};

static SEND_PAYMENT_TEMPLATE : &'static str = r#"[{"address":"{{address}}","value":{{value}} }]"#;

#[derive(Deserialize)]
struct P2SAddress {
    address: String
}

/// Gets node ip from local file `node.ip`
pub fn get_node_ip() -> String {
    let mut file = File::open("node.ip").expect("Failed to open node ip file.");
    let mut st = String::new();
    file.read_to_string(&mut st).ok().expect("Failed to read node ip from file.");
    st.trim().to_string()
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
    let endpoint = get_node_ip() + "/wallet/addresses";
    let client = reqwest::Client::new();
    let hapi_key = HeaderValue::from_str(&api_key).expect("Failed to create header value from api key.");
    let mut res = client.get(&endpoint)
                .header("accept", "application/json")
                .header("api_key", hapi_key)
                .header(CONTENT_TYPE, "application/json")
                .send()
                .expect("Failed to send request to local node. Please make sure it is running on the IP & Port specified in `node.ip` file.");


    let mut addresses : Vec<String> = vec![];
    for segment in res.text().expect("Failed to get addresses from wallet.").split("\""){
        let seg = segment.trim();
        if seg.chars().next().unwrap() == '9' {
           addresses.push(seg.to_string()); 
        }
    }
    if addresses.len() == 0 {
        panic!("No addresses were found. Please make sure it is running on the IP & Port specified in `node.ip` file and that your wallet is unlocked.");
    }
    addresses
}

/// Get P2S Address for Backer to submit to for the Campaign
pub fn get_p2s_address(api_key: &String, campaign: &Campaign,  backer_address: &String) -> String {
    let endpoint = get_node_ip() + "/script/p2sAddress";
    let client = reqwest::Client::new();
    let hapi_key = HeaderValue::from_str(&api_key).expect("Failed to create header value from api key.");
    let mut res = client.post(&endpoint)
                .header("accept", "application/json")
                .header("api_key", hapi_key)
                .header(CONTENT_TYPE, "application/json")
                .body(campaign.build_script(backer_address))
                .send()
                .expect("Failed to send request to local node. Please make sure it is running on the IP & Port specified in `node.ip` file and that your wallet is unlocked.");

    if let Ok(p2saddress) = res.json::<P2SAddress>() {
        return p2saddress.address;
    }
    else if let Err(e) = res.json::<P2SAddress>() {
        println!("{:?}", e);
        let err = res.text().expect("P2S Address error.");
        println!("P2S address node error: {:?}", err);
    }
    panic!("Failed to acquire P2S Address. Make sure your node is running and that the data you provided is valid.");
}

/// Send payment from unlocked wallet to given address via local node api. Returns the box identifier.
pub fn send_wallet_payment(api_key: &String, address: &String, amount: f64) -> Option<BackingTx> {
    let json_body = json!({ "address": address,
                            "value": erg_to_nanoerg(amount) });
    let reg = Handlebars::new();
    let pb = reg.render_template(SEND_PAYMENT_TEMPLATE, &json_body).ok()?;
    let endpoint = get_node_ip() + "/wallet/payment/send";
    let client = reqwest::Client::new();
    let hapi_key = HeaderValue::from_str(&api_key).expect("Failed to create header value from api key.");
    let res = client.post(&endpoint)
                .header("accept", "application/json")
                .header("api_key", hapi_key)
                .header(CONTENT_TYPE, "application/json")
                .body(pb)
                .send();

    let mut tx_id = res.ok()?.text().ok()?;
    tx_id.retain(|c| c != '"');

    if tx_id.contains("bad.request") {
        println!("Failed to make payment. This is the error from the ergo node/wallet:\n{}", tx_id);
        std::process::exit(0);
    }

    return Some(BackingTx::new(tx_id, amount));
}

/// Convert from Erg to nanoErg
pub fn erg_to_nanoerg(erg_amount: f64) -> u64 {
    (erg_amount * 1000000000 as f64) as u64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn erg_conv_is_valid() {
        assert_eq!(1000000000, erg_to_nanoerg(1 as f64));
        assert_eq!(erg_to_nanoerg(3.64), 3640000000);
        assert_eq!(erg_to_nanoerg(0.64), 640000000);
        assert_eq!(erg_to_nanoerg(0.0064), 6400000);
        assert_eq!(erg_to_nanoerg(0.000000064), 64);
        assert_eq!(erg_to_nanoerg(0.000000001), 1);
    }
}