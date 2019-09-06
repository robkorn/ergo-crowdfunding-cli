extern crate handlebars;
#[macro_use]
extern crate serde_json;

mod api_key;
mod campaign;
mod wallet_reqs;

use api_key::{check_for_api_key};
use campaign::{Campaign};
use wallet_reqs::{select_wallet_address};
use docopt::Docopt;
use serde::{Deserialize};
use std::fs::{create_dir};
use std::path::Path;

const USAGE: &'static str = "
Usage: ergo_cf create <project-deadline> <project-goal> 
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_create: bool,
    arg_project_deadline: String,
    arg_project_goal: String,
}

/// Builds the folder structure for local storage
fn build_folder_structure() {
    let mut storage_path = "storage/".to_string();
    create_dir(Path::new(&storage_path)).ok();
    storage_path.push_str("campaigns");
    create_dir(Path::new(&storage_path)).ok();
    let storage_path2 = "storage/backed_campaigns/".to_string();
    create_dir(Path::new(&storage_path2)).ok();
}

/// Ask the user for project name
fn acquire_project_name() -> String {
    println!("Please enter a Crowdfund Campaign name:");
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        input.retain(|c| c != '\n');
        return input;
    }
    println!("Please make sure your name is valid utf-16.");
    return acquire_project_name();
}

// Eventually get backer_pubkey from local node if wallet is unlocked (requires node API key)
pub fn main() {
    println!("Ergo Crowdfund CLI\n------------------");
    build_folder_structure();

    // Get basic values
    let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());
    let api_key = check_for_api_key();

    // Allows you to create a new Crowdfunding Campaign
    if args.cmd_create {
        println!("You are now creating a new Crowdfunding Campaign.");
        let name = acquire_project_name();
        let address = select_wallet_address(&api_key);
        let camp = Campaign::new(&name, &address, &args.arg_project_deadline, &args.arg_project_goal, true);
        camp.save_locally();
    }

    // Allows you to track a Crowdfunding Campaign
    // if args.cmd_track {
        // let camp = Campaign::new(&"First Campaign".to_string(), &args.arg_project_pubkey, &args.arg_project_deadline, &args.arg_project_goal, false);
    // }


    // Allows you to import a Crowdfunding Campaign from a file
    // if args.cmd_import {
        // }

    // Allows you to interact with one of the tracked Crowdfunding Campaigns
    // if args.cmd_interact {
        // }
}


