//! Potential design:
//! The CLI tracks crowdfunding campaigns by storing relevant data in a `.crowdfund` file with the data being in json.
//! This file can be passed around to allow for importing of campaigns, and when imported the CLI app automatically gets a user's public key from their unlocked wallet and then adds that to the json which it stores locally (adds it to the locally tracked campaigns). (Capaign tracking can be added manually as well)
//! Once a campaign is tracked then a user can contribute to it, keep track of previous contributions, search for other people who have contributed, etc. 
//! If the user is the owner of the project_pub_key, then they can withdraw once/if the campaign succeeds.
//! If the user is the owner of backer_pub_key, then they can withdraw once/if the campaign fails.

extern crate handlebars;
#[macro_use]
extern crate serde_json;

mod api_key;
mod campaign;
mod wallet_reqs;

use api_key::{check_for_api_key};
use campaign::*;
use wallet_reqs::{select_wallet_address, get_p2s_address, send_wallet_payment, get_wallet_addresses};
use docopt::Docopt;
use serde::{Deserialize};

const USAGE: &'static str = "
Usage: ergo_cf contribute <backer-pubkey> <project-pubkey> <project-deadline> <project-goal> 
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_contribute: bool,
    arg_backer_pubkey: String,
    arg_project_pubkey: String,
    arg_project_deadline: String,
    arg_project_goal: String,
}

// Eventually get backer_pubkey from local node if wallet is unlocked (requires node API key)
pub fn main() {
    println!("Ergo Crowdfund CLI\n------------------");

    // Get basic values
    let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());
    let api_key = check_for_api_key();

    // If contribute command
    if args.cmd_contribute {
        let camp = Campaign::new(&"First Campaign".to_string(), &args.arg_project_pubkey, &args.arg_project_deadline, &args.arg_project_goal);
        camp.back_campaign(&api_key, 1);
    }
}


