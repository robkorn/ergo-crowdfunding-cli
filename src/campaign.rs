use handlebars::Handlebars;
use crate::wallet_reqs::{select_wallet_address, get_p2s_address, send_wallet_payment};

static CROWDFUND_TEMPLATE : &'static str = r#"{"source": "{ val backerPubKey = PK(\"{{backer}}\") \n val projectPubKey = PK(\"{{project_pub}}\") \n val deadline = {{deadline}} \n val minToRaise = {{amount}}L * 1000000000 \n val fundraisingFailure = HEIGHT >= deadline && backerPubKey \n val enoughRaised = {(outBox: Box) => outBox.value >= minToRaise && outBox.propositionBytes == projectPubKey.propBytes} \n val fundraisingSuccess = HEIGHT < deadline && projectPubKey && OUTPUTS.exists(enoughRaised) \n fundraisingFailure || fundraisingSuccess }"}"#;


/// Datatype which holds relevant information about a Crowdfunding Campaign
pub struct Campaign {
   pub name: String,
   pub project_pub_key: String,
   pub project_deadline: u32,
   pub project_goal: u32
}

/// Datatype which holds a `Campaign` and relevant information about the campaign as a backer. Struct only created after a user has backed a campaign.
pub struct BackedCampaign <'a> {
    pub campaign: &'a Campaign,
    pub backer_pub_key: String,
    pub p2s_address: String,
    pub backer_boxes: Vec<String>
}

impl Campaign {
    /// Create a new `Campaign`. Verifies that the deadline and the goal are valid `u32` integers
    pub fn new (name : &String, project_pub_key: &String, project_deadline: &String, project_goal: &String) -> Campaign{
        let deadline : u32 = project_deadline.parse().expect("Deadline provided is not a valid integer.");
        let goal : u32 = project_goal.parse().expect("Project goal provided is not a valid integer.");
        Campaign {
            name: name.clone(),
            project_pub_key: project_pub_key.clone(),
            project_deadline: deadline,
            project_goal: goal
        } 
    }

    /// Builds the crowdfunding script with the required fields filled in
    pub fn build_script(&self, backer_pub_key: &String) -> String {
        let reg = Handlebars::new();
        let finalized_script = reg.render_template(CROWDFUND_TEMPLATE, 
        &json!({"backer": backer_pub_key
            ,"project_pub": self.project_pub_key
            ,"deadline": self.project_deadline.to_string()
            ,"amount": self.project_goal.to_string()
        }));

        finalized_script.expect("Failed to produce crowdfunding script.")
    }

    /// Allows the user to back the Campaign
    pub fn back_campaign(&self, api_key: &String, amount: u32) -> BackedCampaign {
        let backer_pub_key = select_wallet_address(&api_key);
        let p2s_address = get_p2s_address(&api_key, &self, &backer_pub_key);
        let backer_box_id = send_wallet_payment(&api_key, &p2s_address, amount);

        if let Some(box_id) = backer_box_id {
            let backer_boxes = vec![box_id];
            return BackedCampaign::new(self, backer_pub_key, p2s_address, backer_boxes);
        }
        panic!("Failed to send wallet payment to P2S Address.");
    }
}

impl<'a> BackedCampaign<'a> {
    /// Create a new `BackedCampaign`. 
    pub fn new (campaign : &'a Campaign, backer_pub_key: String, p2s_address: String, backer_boxes: Vec<String>) -> BackedCampaign<'a> { 
        BackedCampaign  {   campaign: campaign,
                            backer_pub_key: backer_pub_key,
                            p2s_address: p2s_address,
                            backer_boxes: backer_boxes
                        }
    }

    // Allow the backer to back the same Campaign again. Creates a new `BackedCampaign` with the new box produced from the new `send_wallet_payment()` added to `backer_boxes` vector.
    // pub fn back_campaign(&self, api_key: &String, amount: u32) -> BackedCampaign {
    // }
}
