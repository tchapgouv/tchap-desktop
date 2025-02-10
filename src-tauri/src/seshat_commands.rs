// use serde::{Serialize, Deserialize};
// use strum_macros::Display;

// use crate::action_handler::ActionHandler;


// #[derive(Serialize, Deserialize, Display)]
// #[serde(rename_all(serialize="camelCase", deserialize="camelCase"), tag = "type", content = "payload")]
// #[strum(serialize_all = "camelCase")]
// pub enum SeshatAction {
// }


// pub const CLASSIFIER_DOMAIN: &str = "seshat";

// pub struct SeshatService {}
// impl SeshatService {
//     pub fn update_classifier_name(&self, new_name: &str) -> () {/* TODO: implement */}
// }


// impl ActionHandler for SeshatService {
//     type TActionType = ClassifierAction;

//     fn domain(&self) -> &str { CLASSIFIER_DOMAIN}
//     fn handle_action(&self, action: Self::TActionType) -> Result<Self::TActionType, serde_json::Error> {
//         Ok("")
//     }
// }