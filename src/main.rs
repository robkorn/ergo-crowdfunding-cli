extern crate handlebars;
#[macro_use]
extern crate serde_json;

mod api_key;
mod campaign;
mod wallet_reqs;

use api_key::{check_for_api_key};
use campaign::{EXPORT_FOLDER, CAMPAIGNS_FOLDER, CrowdfundingCampaign, Campaign, choose_local_campaign};
use crossterm::{terminal,ClearType};
use docopt::Docopt;
use serde::{Deserialize};
use std::fs::{File, create_dir};
use std::io::prelude::*;
use std::path::Path;
use wallet_reqs::{select_wallet_address};

const USAGE: &'static str = "
Usage: 
        ergo_cf back
        ergo_cf create <campaign-name> <campaign-deadline> <campaign-goal>
        ergo_cf delete
        ergo_cf info
        ergo_cf import <file-path>
        ergo_cf export
        ergo_cf track <campaign-name> <campaign-address> <campaign-deadline> <campaign-goal> 
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_back: bool,
    cmd_create: bool,
    cmd_delete: bool,
    cmd_track: bool,
    cmd_info: bool,
    cmd_import: bool,
    cmd_export: bool,
    arg_campaign_name: String,
    arg_campaign_address: String,
    arg_campaign_deadline: String,
    arg_campaign_goal: String,
    arg_file_path: String,
}

/// Builds the folder structure for local storage
fn build_folder_structure() {
    create_dir(Path::new(STORAGE_FOLDER!())).ok();
    create_dir(Path::new(CAMPAIGNS_FOLDER)).ok();
    create_dir(Path::new(EXPORT_FOLDER)).ok();
}

/// Checks if `node.ip` file exists, else creates default one for node at `http://0.0.0.0:9052`
fn generate_default_node_ip_file() {
    let file_path = Path::new("node.ip");
    if file_path.exists() == false {
        let node_ip = "http://0.0.0.0:9052".to_string();
        let mut file = File::create(file_path).expect("Failed to write to node.ip file.");
        file.write_all(&node_ip.into_bytes()).expect("Failed to write node ip to file.");
    }
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
    println!("Ergo Crowdfund CLI Tool\n-----------------------");
}

/// Track Campagin
fn track_campaign(camp: &Campaign, terminal: &crossterm::Terminal) {
    camp.clone().save_locally();
    clear_and_title(&terminal);
    println!("Valid Campaign information submitted. This campaign is now being tracked:\n");
    camp.print_info();

}

/// Back Campaign
fn back_campaign (c: Box<CrowdfundingCampaign>, terminal: &crossterm::Terminal, api_key: &String) {
        c.print_info();
        let back_amount = query_amount();
        clear_and_title(&terminal);
        let backed_camp = c.back_campaign(&api_key, back_amount);
        clear_and_title(&terminal);
        backed_camp.print_info();
}

/// Asks user for an amount
fn query_amount() -> u64 {
    println!("\nHow many Erg do you want to send to this campaign? (Only whole number values for now)");
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input){
        if let Ok(input_n) = input.trim().parse::<u64>(){
            if input_n < 1 {
                println!("Please input a whole number greater than 0.");
                return query_amount();
            }
            return input_n;
        }
    }
    return 0;
}

pub fn main() {
    build_folder_structure();
    generate_default_node_ip_file();

    // Get basic values
    let mut terminal = terminal();
    clear_and_title(&terminal);
    let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());
    let api_key = check_for_api_key();

    clear_and_title(&terminal);

    // Allows you to create a new Crowdfunding Campaign
    if args.cmd_create {
        let address = select_wallet_address(&api_key);
        let camp = Campaign::new(&args.arg_campaign_name, &address, &args.arg_campaign_deadline, &args.arg_campaign_goal);
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
        let text = "see more information about".to_string();
        let camp = choose_local_campaign(&text);
        clear_and_title(&terminal);
        camp.print_info();
    }

    // Allows you to import a Crowdfunding Campaign from a file
    if args.cmd_import {
        let camp = Campaign::from_file(&args.arg_file_path);
        track_campaign(&camp, &terminal);
    }

    // Allows you to export a Crowdfunding Campaign to a file
    if args.cmd_export {
        let text = "export".to_string();
        let camp = choose_local_campaign(&text);
        camp.export();
    }


    // Allows deletion of tracked Campaign
    if args.cmd_delete {
        let text = "delete".to_string();
        let camp = choose_local_campaign(&text);
        camp.delete();
    }

    // Allows you to back one of the tracked Crowdfunding Campaigns
    // Eventually implement a Campaign trait to not have code repeat
    if args.cmd_back {
        let text = "back".to_string();
        let camp = choose_local_campaign(&text);
        clear_and_title(&terminal);
        back_campaign(camp, &terminal, &api_key);
    }
}


