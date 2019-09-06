extern crate handlebars;
#[macro_use]
extern crate serde_json;

mod api_key;
mod campaign;
mod wallet_reqs;

use api_key::{check_for_api_key};
use campaign::{EXPORT_FOLDER ,CAMPAIGNS_FOLDER, BACKED_CAMPAIGNS_FOLDER, Campaign, choose_local_campaign};
use crossterm::{terminal,ClearType};
use docopt::Docopt;
use serde::{Deserialize};
use std::fs::{create_dir};
use std::path::Path;
use wallet_reqs::{select_wallet_address};

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
    create_dir(Path::new("storage/")).ok();
    create_dir(Path::new(CAMPAIGNS_FOLDER)).ok();
    create_dir(Path::new(BACKED_CAMPAIGNS_FOLDER)).ok();
    create_dir(Path::new(EXPORT_FOLDER)).ok();
}

/// Ask the user for project name
fn acquire_project_name() -> String {
    println!("Please enter a name for your new Crowdfund Campaign:");
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        input.retain(|c| c != '\n');
        return input;
    }
    println!("Please make sure your name is valid utf-16.");
    return acquire_project_name();
}

fn clear_and_title(terminal: &crossterm::Terminal) {
    terminal.clear(ClearType::All);
    println!("Ergo Crowdfund CLI\n------------------");
}

// Eventually get backer_pubkey from local node if wallet is unlocked (requires node API key)
pub fn main() {
    build_folder_structure();

    // Get basic values
    let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());
    let api_key = check_for_api_key();
    let mut terminal = terminal();

    clear_and_title(&terminal);

    // Allows you to create a new Crowdfunding Campaign
    if args.cmd_create {
        let name = acquire_project_name();
        let address = select_wallet_address(&api_key);
        let camp = Campaign::new(&name, &address, &args.arg_project_deadline, &args.arg_project_goal);
        camp.clone().save_locally();
        camp.clone().export();
        clear_and_title(&terminal);
        println!("Your campaign has been created.\nCheck out the 'export' folder to share the campaign file with others.\n");
        camp.print_info();
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


