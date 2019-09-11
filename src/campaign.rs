use crate::wallet_reqs::{select_wallet_address, get_p2s_address, send_wallet_payment};
use handlebars::Handlebars;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs::{File, read_dir, remove_file};

static CROWDFUND_TEMPLATE : &'static str = r#"{"source": "{ val backerPubKey = PK(\"{{backer}}\") \n val projectPubKey = PK(\"{{address}}\") \n val deadline = {{deadline}} \n val minToRaise = {{goal}}L * 1000000000 \n val fundraisingFailure = HEIGHT >= deadline && backerPubKey \n val enoughRaised = {(outBox: Box) => outBox.value >= minToRaise && outBox.propositionBytes == projectPubKey.propBytes} \n val fundraisingSuccess = HEIGHT < deadline && projectPubKey && OUTPUTS.exists(enoughRaised) \n fundraisingFailure || fundraisingSuccess }"}"#;

#[macro_export]
macro_rules! STORAGE_FOLDER {() => ( ".storage/" )}
pub static CAMPAIGNS_FOLDER : &'static str = concat!(STORAGE_FOLDER!(), "campaigns/");
pub static EXPORT_FOLDER : &'static str = "export/";


pub trait CrowdfundingCampaign {
    fn back_campaign(&self, api_key: &String, amount: u64) -> BackedCampaign;
    fn build_script(&self, backer_address: &String) -> String;
    fn export(&self);
    fn delete(&self);
    fn save(&self, path: &mut String);
    fn save_locally(&self);
    fn print_info(&self);
    fn print_choice_text(&self, n: u32);
}

/// Datatype which holds relevant information about a Crowdfunding Campaign.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Campaign {
   pub name: String,
   pub address: String,
   pub deadline: u64,
   pub goal: u64,
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
    pub backed_amount: u64
}

impl Campaign {
    /// Create a new `Campaign`. Verifies that the deadline and the goal are valid `u64` integers
    pub fn new (name : &String, address: &String, deadline: &String, goal: &String) -> Campaign{
        let deadline : u64 = deadline.parse().expect("Deadline provided is not a valid integer.");
        let goal : u64 = goal.parse().expect("campaign goal provided is not a valid integer.");
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
}


impl CrowdfundingCampaign for Campaign {
    /// Builds the crowdfunding script with the required fields filled in
    fn build_script(&self, backer_address: &String) -> String {
        let reg = Handlebars::new();
        let finalized_script = reg.render_template(CROWDFUND_TEMPLATE, 
        &json!({"backer": backer_address
            ,"address": self.address
            ,"deadline": self.deadline.to_string()
            ,"goal": self.goal.to_string()
        }));

        finalized_script.expect("Failed to produce crowdfunding script.")
    }

    /// Saves `Campaign` to path
    fn save(&self, path: &mut String) {
        path.push_str(&self.name);
        path.push_str(".campaign");
        clean_path_name(path);
        let file = File::create(path.trim()).expect("Failed to create Campaign file.");
        serde_json::to_writer_pretty(file, &self).expect("Failed to save Campaign to file.");
    }

    /// Save the `Campaign` locally into a json file in the Campaigns folder
    fn save_locally(&self) {
        let mut path = CAMPAIGNS_FOLDER.to_string();
        self.save(&mut path);
    }

    /// Exports the `Campaign` into a json file to be shared in the export folder
    fn export(&self) {
        let mut path = EXPORT_FOLDER.to_string();
        self.save(&mut path);
    }

    /// Deletes the tracked Campaign from local storage
    fn delete (&self) {
        let mut path = CAMPAIGNS_FOLDER.to_string();
        path.push_str(&self.name);
        path.push_str(".campaign");
        clean_path_name(&mut path);
        remove_file(path).ok();
    }

    /// Allows the user to back the Campaign
    fn back_campaign(&self, api_key: &String, amount: u64) -> BackedCampaign {
        let backer_address = select_wallet_address(&api_key);
        let p2s_address = get_p2s_address(&api_key, &self, &backer_address);
        let backing_tx = send_wallet_payment(&api_key, &p2s_address, amount);

        if let Some(bt) = backing_tx {
            let backer_txs = vec![bt];
            let new_camp = Campaign::new(&self.name.clone(), &self.address.clone(), &self.deadline.to_string(), &self.goal.to_string());
            let backed_camp = BackedCampaign::new(new_camp, backer_address, p2s_address, backer_txs);
            backed_camp.clone().save_locally();
            return backed_camp;
        }
        panic!("Failed to send wallet payment to P2S Address.");
    }

    /// Prints info about the Campaign
    fn print_info(&self) {
        println!("Campaign Name: {}\nCampaign Address: {}\nCampaign Deadline Block: {}\nCampaign Goal: {}", self.name, self.address, self.deadline, self.goal);
    }

    fn print_choice_text(&self, n: u32) {
        println!("{}. {}", n, self.name);

    }
}


impl CrowdfundingCampaign for BackedCampaign {

    fn print_choice_text(&self, n: u32) {
        println!("{}. {} - (You Backed This Campaign Previously)", n, self.campaign.name);
    }

    fn build_script(&self, backer_address: &String) -> String {
        self.campaign.build_script(backer_address)
    }

    /// Saves the `BackedCampaign` to path
    fn save(&self, path: &mut String) {
        path.push_str(&self.campaign.name);
        path.push_str(".campaign");
        clean_path_name(path);
        let file = File::create(path.trim()).expect("Failed to create Backed Campaign file.");
        serde_json::to_writer_pretty(file, &self).expect("Failed to save Backed Campaign to file.");
        println!("Campaign saved locally.");
    }

    /// Save the `BackedCampaign` locally into a json file in the Campaigns folder
    fn save_locally(&self) {
        let mut path = CAMPAIGNS_FOLDER.to_string();
        self.save(&mut path);
    }

    /// Exports the `Campaign` from the `BackedCampaign` to Export folder
    fn export(&self) {
        self.campaign.export();
    }

    /// Deletes the tracked Campaign from local storage
    fn delete (&self) {
        self.campaign.delete();
    }

    // Allow the backer to back the same Campaign again. Creates a new `BackedCampaign` with the new `BackingTx` produced from the new `send_wallet_payment()` added to `backer_txs` vector.
    fn back_campaign(&self, api_key: &String, amount: u64) -> BackedCampaign {
        let backer_address = select_wallet_address(&api_key);
        let p2s_address = get_p2s_address(&api_key, &self.campaign, &backer_address);
        let backing_tx = send_wallet_payment(&api_key, &p2s_address, amount);

        if let Some(bt) = backing_tx {
            let mut backer_txs = self.backer_txs.clone();
            backer_txs.push(bt);
            let backed_camp = BackedCampaign::new(self.campaign.clone(), backer_address, p2s_address, backer_txs);
            backed_camp.clone().save_locally();
            return backed_camp;
        }
        panic!("Failed to send wallet payment to P2S Address.");

    }

    /// Prints info about the `BackedCampaign`
    fn print_info(&self) {
        self.campaign.print_info();
        println!("Address You Used To Back: {}\nP2S Address Paid To: {}\nBacking Txs:", self.backer_address, self.p2s_address);
        for tx in &self.backer_txs{
            println!("   - {}: {} Erg", tx.tx_id, tx.backed_amount);
        }
    }
}

impl BackingTx {
    pub fn new(tx_id: String, backed_amount: u64) -> BackingTx {
        BackingTx {tx_id: tx_id, backed_amount: backed_amount}
    }
}

/// Choose a campaign from those which are locally saved
pub fn choose_local_campaign(action_string: &String) -> Box<CrowdfundingCampaign> {
    let mut camps = get_local_campaigns();
    if camps.len() == 0 {
        println!("You have no local Campaigns. Please create or track a Campaign first to interact with one."); 
        std::process::exit(0);
    }
    let mut n = 0;
    for camp in &camps {
        n += 1;
        camp.print_choice_text(n);
    }

    // Making the campaign selection text more context dependent.
    println!("\nWhich campaign would you like to {}?", action_string);


    // Get Campaign based on choice
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input){
        if let Ok(input_n) = input.trim().parse::<usize>(){
            if input_n > get_local_campaigns().len() || input_n < 1 {
                println!("Please select a campaign within the range.");
                return choose_local_campaign(action_string);
            }
            return camps.remove(input_n-1);
        }
    }
    return choose_local_campaign(action_string);
}

/// Get a vector of the locally stored `Campaign`s and `BackedCampaign`s
pub fn get_local_campaigns() -> Vec<Box<CrowdfundingCampaign>> {
    let mut campaigns : Vec<Box<CrowdfundingCampaign>> = vec![];
    let path = Path::new(CAMPAIGNS_FOLDER);
    if let Ok(rd) = read_dir(path){
        for rentry in rd {
            if let Ok(entry) = rentry {
                let file = File::open(entry.path()).expect("Failed to read Campaign json file.");
                let campaign : Option<Campaign> =  serde_json::from_reader(file).ok();
                if let Some(camp) = campaign {
                    campaigns.push(Box::new(camp));
                    continue;
                }
                let file = File::open(entry.path()).expect("Failed to read Campaign json file.");
                let backedcampaign : Option<BackedCampaign> = serde_json::from_reader(file).ok();
                if let Some(backed_camp) = backedcampaign {
                    campaigns.push(Box::new(backed_camp));
                }
            }
        }
    }
    campaigns
}


fn clean_path_name (path: &mut String) {
    path.retain(|c| c != '\n' && c != ' ')
}