use crate::wallet_reqs::{select_wallet_address, get_p2s_address, send_wallet_payment};
use handlebars::Handlebars;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs::{File, read_dir};

static CROWDFUND_TEMPLATE : &'static str = r#"{"source": "{ val backerPubKey = PK(\"{{backer}}\") \n val projectPubKey = PK(\"{{project_adress}}\") \n val deadline = {{deadline}} \n val minToRaise = {{amount}}L * 1000000000 \n val fundraisingFailure = HEIGHT >= deadline && backerPubKey \n val enoughRaised = {(outBox: Box) => outBox.value >= minToRaise && outBox.propositionBytes == projectPubKey.propBytes} \n val fundraisingSuccess = HEIGHT < deadline && projectPubKey && OUTPUTS.exists(enoughRaised) \n fundraisingFailure || fundraisingSuccess }"}"#;

pub static CAMPAIGNS_FOLDER : &'static str = "storage/campaigns/";
pub static BACKED_CAMPAIGNS_FOLDER : &'static str = "storage/backed_campaigns/";
pub static EXPORT_FOLDER : &'static str = "export/";


/// Enum for returning either a backed or not backed Campaign
#[derive(Clone)]
pub enum Camp {
    Backed(BackedCampaign),
    NotBacked(Campaign)
}

/// Datatype which holds relevant information about a Crowdfunding Campaign.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Campaign {
   pub name: String,
   pub address: String,
   pub deadline: u32,
   pub goal: u32,
}

/// Datatype which holds a `Campaign` and relevant information about the campaign as a backer. Struct only created after a user has backed a campaign.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackedCampaign {
    pub campaign: Campaign,
    pub backer_address: String,
    pub p2s_address: String,
    pub backer_txs: Vec<BackingTx>
}

/// Datatype which holds information about a backer's transaction to support a Campaign.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackingTx {
    pub tx_id: String,
    pub backed_amount: u32
}

impl Campaign {
    /// Create a new `Campaign`. Verifies that the deadline and the goal are valid `u32` integers
    pub fn new (name : &String, address: &String, deadline: &String, goal: &String) -> Campaign{
        let deadline : u32 = deadline.parse().expect("Deadline provided is not a valid integer.");
        let goal : u32 = goal.parse().expect("campaign goal provided is not a valid integer.");
        Campaign {
            name: name.clone(),
            address: address.clone(),
            deadline: deadline,
            goal: goal,
        } 
    }

    /// Create a new `Campaign` from a previously exported `Campaign`
    pub fn from_file (path: &String) -> Campaign {
        let file = File::open(path).expect("Failed to read Campaign file.");
        serde_json::from_reader(file).expect("Failed to process Campaign from json.")
    }

    /// Builds the crowdfunding script with the required fields filled in
    pub fn build_script(&self, backer_address: &String) -> String {
        let reg = Handlebars::new();
        let finalized_script = reg.render_template(CROWDFUND_TEMPLATE, 
        &json!({"backer": backer_address
            ,"address": self.address
            ,"deadline": self.deadline.to_string()
            ,"amount": self.goal.to_string()
        }));

        finalized_script.expect("Failed to produce crowdfunding script.")
    }

    /// Allows the user to back the Campaign
    pub fn back_campaign(self, api_key: &String, amount: u32) -> BackedCampaign {
        let backer_address = select_wallet_address(&api_key);
        let p2s_address = get_p2s_address(&api_key, &self, &backer_address);
        let backing_tx = send_wallet_payment(&api_key, &p2s_address, amount);

        if let Some(bt) = backing_tx {
            let backer_txs = vec![bt];
            return BackedCampaign::new(self, backer_address, p2s_address, backer_txs);
        }
        panic!("Failed to send wallet payment to P2S Address.");
    }

    /// Saves `Campaign` to path
    fn save(self, path: &mut String) {
        path.push_str(&self.name);
        path.push_str(".campaign");
        path.retain(|c| c != '\n' && c != ' ');
        let file = File::create(path.trim()).expect("Failed to create Campaign file.");
        serde_json::to_writer_pretty(file, &self).expect("Failed to save Campaign to file.");
    }

    /// Save the `Campaign` locally into a json file in the `storage/campaigns/` folder
    pub fn save_locally(self) {
        let mut path = CAMPAIGNS_FOLDER.to_string();
        self.save(&mut path);
    }

    /// Exports the `Campaign` into a json file to be shared in the `export/` folder
    pub fn export(self) {
        let mut path = EXPORT_FOLDER.to_string();
        self.save(&mut path);
    }

    /// Prints info about the Campaign
    pub fn print_info(&self) {
        println!("Local Campaign Name: {}\nCampaign Address: {}\nCampaign Deadline Block: {}\nCampaign Goal: {}", self.name, self.address, self.deadline, self.goal);
    }
}

impl BackedCampaign {
    /// Create a new `BackedCampaign`. 
    pub fn new (campaign : Campaign, backer_address: String, p2s_address: String, backer_txs: Vec<BackingTx>) -> BackedCampaign { 
        BackedCampaign  {   campaign: campaign,
                            backer_address: backer_address,
                            p2s_address: p2s_address,
                            backer_txs: backer_txs
                        }
    }

    // Allow the backer to back the same Campaign again. Creates a new `BackedCampaign` with the new `BackingTx` produced from the new `send_wallet_payment()` added to `backer_txs` vector.
    // pub fn back_campaign(&self, api_key: &String, amount: u32) -> BackedCampaign {
    // }


    /// Saves the `BackedCampaign` to path
    fn save(self, path: &mut String) {
        path.push_str(&self.campaign.name);
        path.push_str(".campaign");
        path.retain(|c| c != '\n' && c != ' ');
        let file = File::create(path.trim()).expect("Failed to create Backed Campaign file.");
        serde_json::to_writer_pretty(file, &self).expect("Failed to save Backed Campaign to file.");
        println!("Campaign saved locally.");
    }

    /// Save the `BackedCampaign` locally into a json file in the `storage/backed_campaigns/` folder
    pub fn save_locally(self) {
        let mut path = BACKED_CAMPAIGNS_FOLDER.to_string();
        self.save(&mut path);
    }

    /// Exports the `Campaign` from the `BackedCampaign` to `export/` folder
    pub fn export(self) {
        self.campaign.export();
    }
}

impl BackingTx {
    pub fn new(tx_id: String, backed_amount: u32) -> BackingTx {
        BackingTx {tx_id: tx_id, backed_amount: backed_amount}
    }
}

/// Choose a campaign from those which are locally saved
pub fn choose_local_campaign() -> Camp {
    let camps = get_local_campaigns();
    if camps.len() == 0 {
        println!("You have no local Campaigns. Please create or track a Campaign first to interact with one."); 
        std::process::exit(0);
    }
    let mut n = 0;
    for camp in &camps {
        n += 1;
        if let Camp::Backed(bc) = camp {
            println!("{}. {} - (You Backed This Campaign Previously)", n, bc.campaign.name);
        }
        else if let Camp::NotBacked(c) = camp {
            println!("{}. {}", n, c.name);
        }
    }
    println!("\nWhich campaign would you like to select?");
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input){
        if let Ok(input_n) = input.trim().parse::<usize>(){
            if input_n > get_local_campaigns().len() || input_n < 1 {
                println!("Please select a campaign within the range.");
                return choose_local_campaign();
            }
            return camps[input_n-1].clone();
        }
    }
    return choose_local_campaign();

}

/// Get a vector of the locally stored `Campaign`s and `BackedCampaign`s
pub fn get_local_campaigns() -> Vec<Camp> {
    let mut campaigns = vec![];
    let path = Path::new(CAMPAIGNS_FOLDER);
    if let Ok(rd) = read_dir(path){
        for rentry in rd {
            if let Ok(entry) = rentry {
                let file = File::open(entry.path()).expect("Failed to read Campaign json file.");
                let camp : Campaign = serde_json::from_reader(file).expect("Failed to process Campaign from json file.");
                campaigns.push(Camp::NotBacked(camp));
            }
        }
    }
    let path = Path::new(BACKED_CAMPAIGNS_FOLDER);
    if let Ok(rd) = read_dir(path){
        for rentry in rd {
            if let Ok(entry) = rentry {
                let file = File::open(entry.path()).expect("Failed to read Backed Campaign json file.");
                let backed_camp : BackedCampaign = serde_json::from_reader(file).expect("Failed to process Backed Campaign from json file.");
                campaigns.push(Camp::Backed(backed_camp));
            }
        }
    }
    campaigns
}
