use serde::{Serialize, Deserialize};
use strum_macros::Display;

use crate::action_handler::ActionHandler;


#[derive(Serialize, Deserialize, Display)]
#[serde(rename_all(serialize="camelCase", deserialize="camelCase"), tag = "type", content = "payload")]
#[strum(serialize_all = "camelCase")]
pub enum CommonAction {
    setHomeserverUrl(String),
    getAppVersion()
}

// we need one file with constants, like
// this will also be generated for redux to use a slice name
pub const COMMON_DOMAIN: &str = "common";

pub struct CommonService {}
impl CommonService {
    pub fn set_homeserver_url(&self, url: &str) -> () {
        /* TODO: implement */
        println!(url);
    }
}


impl ActionHandler for CommonService {
    type TActionType = CommonAction;

    fn domain(&self) -> &str { COMMON_DOMAIN }

    fn handle_action(&self, action: Self::TActionType) -> Result<Self::TActionType, serde_json::Error> {

        let response = match action {
            CommonAction::setHomeserverUrl(data) => {
                self.set_homeserver_url(&data.new_name);
            }
        };
        Ok(response)
    }
}