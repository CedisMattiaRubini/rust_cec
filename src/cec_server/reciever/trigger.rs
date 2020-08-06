use serde::{Serialize, Deserialize};

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
            let trigger = match super::get_json_string(&json, &"trigger"){
                Some(trigger) => trigger,
                None => return Err(super::ServerError::InvalidJson),
            };
            let response_id = match super::get_json_string(&json, &"response_id"){
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

}

impl std::fmt::Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "trigger: {}, delay: {}", self.trigger, self.response_id)
    }
}