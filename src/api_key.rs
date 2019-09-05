use std::fs::{File};
use std::io::prelude::*;


/// Saves a provided api key to `api.key` file
fn save_api_key_to_file(api_key: &String) {
    let mut file = File::create("api.key").expect("Failed to write to api.key file.");
    file.write_all(&api_key.clone().into_bytes()).expect("Failed to write keypair to file.");
}

/// Gets an api key from `api.key` file
fn get_api_key_from_file() -> Option<String> {
    let mut file = File::open("api.key").ok()?;
    let mut st = String::new();
    file.read_to_string(&mut st).ok()?;
    Some(st)
}

/// Tries to get api key from `api.key` file, else asks the user to enter their api key and saves it to `api.key`
pub fn check_for_api_key() -> String {
    if let Some(api_key) = get_api_key_from_file(){
        return api_key;
    }
    else {
        println!("You do not have your node api key saved for use with this CLI app.\nPlease enter it now:");
        let mut input = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut input){
            println!("API key inputted: {}", input);
            save_api_key_to_file(&input);
        }
        else {
            panic!("Provided invalid input. Please relaunch the CLI app and try again with a valid API key.")
        }
        return input;
    }
}