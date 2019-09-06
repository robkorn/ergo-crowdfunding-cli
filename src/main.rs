extern crate handlebars;
#[macro_use]
extern crate serde_json;

mod api_key;
mod campaign;
mod wallet_reqs;

use api_key::{check_for_api_key};
use campaign::{EXPORT_FOLDER ,CAMPAIGNS_FOLDER, BACKED_CAMPAIGNS_FOLDER, Campaign, Camp, choose_local_campaign};
use crossterm::{terminal,ClearType};
use docopt::Docopt;
use serde::{Deserialize};
use std::fs::{create_dir};
use std::path::Path;
use wallet_reqs::{select_wallet_address};

const USAGE: &'static str = "
Usage: 
        ergo_cf create <campaign-deadline> <campaign-goal> 
        ergo_cf info
        ergo_cf track <campaign-name> <campaign-address> <campaign-deadline> <campaign-goal> 
        ergo_cf import <file-path>
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_create: bool,
    cmd_track: bool,
    cmd_info: bool,
    cmd_import: bool,
    arg_campaign_name: String,
    arg_campaign_address: String,
    arg_campaign_deadline: String,
    arg_campaign_goal: String,
    arg_file_path: String,
}

/// Builds the folder structure for local storage
fn build_folder_structure() {
    create_dir(Path::new("storage/")).ok();
    create_dir(Path::new(CAMPAIGNS_FOLDER)).ok();
    create_dir(Path::new(BACKED_CAMPAIGNS_FOLDER)).ok();
    create_dir(Path::new(EXPORT_FOLDER)).ok();
}

/// Ask the user for campaign name
fn acquire_campaign_name() -> String {
    println!("Please enter a name for your new Crowdfund Campaign:");
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        input.retain(|c| c != '\n');
        return input;
    }
    println!("Please make sure your name is valid utf-16.");
    return acquire_campaign_name();
}

/// Clear terminal screen and print title
fn clear_and_title(terminal: &crossterm::Terminal) {
    terminal.clear(ClearType::All);
    println!("Ergo Crowdfund CLI\n------------------");
}

/// Track Campagin
fn track_campaign(camp: &Campaign, terminal: &crossterm::Terminal){
    camp.clone().save_locally();
    clear_and_title(&terminal);
    println!("Valid Campaign information submitted. This campaign is now being tracked:\n");
    camp.print_info();

}

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
        let name = acquire_campaign_name();
        let address = select_wallet_address(&api_key);
        let camp = Campaign::new(&name, &address, &args.arg_campaign_deadline, &args.arg_campaign_goal);
        camp.clone().save_locally();
        camp.clone().export();
        clear_and_title(&terminal);
        println!("Your campaign has been created.\nCheck out the 'export' folder to share the campaign file with others.\n");
        camp.print_info();
    }

    // Allows you to track a Crowdfunding Campaign
    if args.cmd_track {
        let camp = Campaign::new(&args.arg_campaign_name, &args.arg_campaign_address, &args.arg_campaign_deadline, &args.arg_campaign_goal);
        track_campaign(&camp, &terminal);
    }

    // Provides info about a tracked Crowdfunding Campaign
    if args.cmd_info {
        let camp = choose_local_campaign();
        clear_and_title(&terminal);
        match camp {
            Camp::NotBacked(c) => c.print_info(),
            Camp::Backed(bc) => bc.campaign.print_info()
        }
    }


    // Allows you to import a Crowdfunding Campaign from a file
    if args.cmd_import {
        let camp = Campaign::from_file(&args.arg_file_path);
        track_campaign(&camp, &terminal);
    }

    // Allows you to interact with one of the tracked Crowdfunding Campaigns
    // if args.cmd_back {
        // choose_local_campaign();
        // }
}


