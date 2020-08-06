use std::time;

use serde::{Serialize, Deserialize};

/// Contains a command to send, and an eventual delay to respect
#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub(super) cmd: String,
    pub(super) delay: Option<time::Duration>,
    
}

impl Response {

    pub fn new(cmd: String, delay: Option<time::Duration>) -> Response {
        Response {
            cmd: cmd,
            delay: delay,
        }
    }

    pub fn from_json(json: &serde_json::Value) -> super::Result<Response> {
        // Delay can be null
        if json["cmd"].is_string() {
            let cmd = match super::get_json_string(&json, &"cmd"){
                Some(cmd) => cmd,
                None => return Err(super::ServerError::InvalidJson),
            };
            Ok(Response{
                cmd: cmd,
                delay: super::json_duration(&json["delay"]),
            })
        } else {
            Err(super::ServerError::InvalidJson)
        }
    }

}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let delay = match self.delay {
            Some(delay) => delay.as_secs(),
            None => 0,
        };
        write!(f, "cmd: {}, delay: {}", self.cmd, delay)
    }
}