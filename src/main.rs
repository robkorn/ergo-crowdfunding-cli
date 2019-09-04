extern crate handlebars;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;

static CROWDFUND_TEMPLATE : &'static str = r#"{"source": "{ val backerPubKey = PK(\"{{backer}}\") \n val projectPubKey = PK(\"{{project_pub}}\") \n val deadline = {{deadline}} \n val minToRaise = {{amount}}L * 1000000000 \n val fundraisingFailure = HEIGHT >= deadline && backerPubKey \n val enoughRaised = {(outBox: Box) => outBox.value >= minToRaise && outBox.propositionBytes == projectPubKey.propBytes} \n val fundraisingSuccess = HEIGHT < deadline && projectPubKey && OUTPUTS.exists(enoughRaised) \n fundraisingFailure || fundraisingSuccess }"}"#;


/// Builds the crowdfunding script with the required fields filled in
/// (Still need to add a few more fields)
fn build_crowdfund_script (backer_pub_key: &String, project_pub_key: &String) -> Option<String> {
     let reg = Handlebars::new();
     let finalized_script = reg.render_template(CROWDFUND_TEMPLATE, 
     &json!({"backer": backer_pub_key
            ,"project_pub": project_pub_key
            ,"deadline": "50000"         
            ,"amount": "500"
     })).ok();
    println!("{}", &finalized_script.to_owned()?);

    finalized_script
}

pub fn main() {
    let backer_pub_key = "9h7DHKSDgE4uvP8313GVGdsEg3AvdAWSSTG7XZsLwBfeth4aePG".to_string();
    let project_pub_key = "9gBSqNT9LH9WjvWbyqEvFirMbYp4nfGHnoWdceKGu45AKiya3Fq".to_string();
    build_crowdfund_script(&backer_pub_key, &project_pub_key);
    ()
}
