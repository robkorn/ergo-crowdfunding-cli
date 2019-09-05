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

// Eventually get backer_pubkey from local node if wallet is unlocked
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
