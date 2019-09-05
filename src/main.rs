//! Potential design:
//! The CLI tracks crowdfunding campaigns by storing relevant data in a `.crowdfund` file with the data being in json.
//! This file can be passed around to allow for importing of campaigns, and when imported the CLI app automatically gets a user's public key from their unlocked wallet and then adds that to the json which it stores locally (adds it to the locally tracked campaigns). (Capaign tracking can be added manually as well)
//! Once a campaign is tracked then a user can contribute to it, keep track of previous contributions, search for other people who have contributed, etc. 
//! If the user is the owner of the project_pub_key, then they can withdraw once/if the campaign succeeds.
//! If the user is the owner of backer_pub_key, then they can withdraw once/if the campaign fails.

extern crate handlebars;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;
use docopt::Docopt;
use serde::{Deserialize};

static CROWDFUND_TEMPLATE : &'static str = r#"{"source": "{ val backerPubKey = PK(\"{{backer}}\") \n val projectPubKey = PK(\"{{project_pub}}\") \n val deadline = {{deadline}} \n val minToRaise = {{amount}}L * 1000000000 \n val fundraisingFailure = HEIGHT >= deadline && backerPubKey \n val enoughRaised = {(outBox: Box) => outBox.value >= minToRaise && outBox.propositionBytes == projectPubKey.propBytes} \n val fundraisingSuccess = HEIGHT < deadline && projectPubKey && OUTPUTS.exists(enoughRaised) \n fundraisingFailure || fundraisingSuccess }"}"#;

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

/// Datatype which holds relevant information about a Crowdfunding Campaign
struct Campaign {
    name: String,
    project_pub_key: String,
    project_deadline: u32,
    project_goal: u32
}

/// Datatype which holds a `Campaign` and relevant information about the campaign as a backer
struct BackedCampaign {
    campaign: Campaign,
    backer_pub_key: String,
    p2s_address: String,
}

impl Campaign {
    /// Create a new `Campaign`. Verifies that the deadline and the goal are valid `u32` integers
    fn new (&self, name : &String, project_pub_key: &String, project_deadline: &String, project_goal: &String) -> Campaign{
        let deadline : u32 = project_deadline.parse().expect("Deadline provided is not a valid integer.");
        let goal : u32 = project_goal.parse().expect("Project goal provided is not a valid integer.");
        Campaign {
            name: name.clone(),
            project_pub_key: project_pub_key.clone(),
            project_deadline: deadline,
            project_goal: goal
        } 
    }
}


impl BackedCampaign {

}


/// Builds the crowdfunding script with the required fields filled in
fn build_crowdfund_script (backer_pub_key: &String, project_pub_key: &String, project_deadline: &String, goal: &String) -> Option<String> {
    let reg = Handlebars::new();
    let finalized_script = reg.render_template(CROWDFUND_TEMPLATE, 
    &json!({"backer": backer_pub_key
           ,"project_pub": project_pub_key
           ,"deadline": project_deadline
           ,"amount": goal
    })).ok();
   println!("{}", &finalized_script.to_owned()?);

   finalized_script
}

/// Build a default crowfund script for testing
fn default_crowdfund_test_script() -> Option<String> {
    let backer_pub_key = "9h7DHKSDgE4uvP8313GVGdsEg3AvdAWSSTG7XZsLwBfeth4aePG".to_string();
    let project_pub_key = "9gBSqNT9LH9WjvWbyqEvFirMbYp4nfGHnoWdceKGu45AKiya3Fq".to_string();
    build_crowdfund_script(&backer_pub_key, &project_pub_key, &"50000".to_string(), &"500".to_string())
}

// Eventually get backer_pubkey from local node if wallet is unlocked (requires node API key)
pub fn main() {
    println!("Ergo Crowdfund CLI\n------------------");
    let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

    // If contribute command
    if args.cmd_contribute {
       build_crowdfund_script(&args.arg_backer_pubkey, &args.arg_project_pubkey, &args.arg_project_deadline, &args.arg_project_goal);
    }
}


