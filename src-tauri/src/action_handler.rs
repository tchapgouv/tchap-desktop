use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

// trait that must be implemented by every domain service
pub trait ActionHandler {
    // specifies the domain actions this trait can handle
    type TAction: DeserializeOwned + Serialize + std::fmt::Display;
​
    // the domain for which this handler is responsible
    fn domain(&self) -> &str;
    
    // must be implemented by derived structs
    fn handle_action(&self, action: Self::TAction) -> 
    Result<Self::TAction, serde_json::Error>;    
    
    // boiler plate code for converting actions to and from json  
    fn receive_action(&self, json_action: Value) -> 
    Result<Value, serde_json::Error> {
        // convert json to action
        let incoming: Self::TAction = serde_json::from_value(json_action)?;
        // call action specific handler
        let response = self.handle_action(incoming)?;
        // convert response to json
        let response_json = serde_json::to_value(response)?;
        Ok(response_json)
    }
}