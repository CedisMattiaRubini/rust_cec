use serde::{Serialize, Deserialize};

use super::jsonparser;

/// Trigger to respond to when recieving a command
#[derive(Deserialize, Serialize, Debug)]
pub struct Trigger {
    pub(super) trigger: String,
    pub(super) response_id: String,
}

impl Trigger {

    pub fn new(trigger: String, response_id: String) -> Trigger {
        Trigger {
            trigger: trigger,
            response_id: response_id,
        }
    }

    /// Convert a json in a Trigger
    pub fn from_json(json: &serde_json::Value) -> super::Result<Trigger> {
        if json["trigger"].is_string() && json["response_id"].is_string(){
            let trigger = match jsonparser::get_json_string(&json, &"trigger"){
                Some(trigger) => trigger,
                None => return Err(super::ServerError::InvalidJson),
            };
            let response_id = match jsonparser::get_json_string(&json, &"response_id"){
                Some(response_id) => response_id,
                None => return Err(super::ServerError::InvalidJson),
            };
            Ok(Trigger{
                trigger: trigger,
                response_id: response_id,
            })
        } else {
            Err(super::ServerError::InvalidJson)
        }
    }

    /// Recieve data, checks if its launch the trigger
    pub(super) fn has_trigger(&self, data: &String) -> Option<&String> {
        if data.eq(&self.trigger){
            return Some(&self.response_id)
        }
        None
    }

}

impl std::fmt::Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "trigger: {}, delay: {}", self.trigger, self.response_id)
    }
}