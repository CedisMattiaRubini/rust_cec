/// The reciever feeds on the stdout of the cec server
/// A get_rsponse can be used to automatically respond with some command to some output

/// The reciever take care of reading the cec output get_rsponseing and sending it where it has to go
use std::time;

use super::{Result, ServerError};

pub mod responcesgroups;
pub mod responces;
pub mod trigger;
pub mod daemon;

pub use daemon::launch;

/// check if the json is compatible with time::duration
fn is_duration(json: &serde_json::value::Value) -> bool {
    !json.is_null() && json["nanos"].is_number() && json["secs"].is_number()
}

/// convert the json (if compatible) with time::duration
fn json_duration(json: &serde_json::value::Value) -> Option<time::Duration>{
    if !is_duration(json) {
        return None
    };
    let secs = match json["secs"].as_u64() {
        None => return None,
        Some(secs) => secs,
    };
    let nanos: u32 = match json["nanos"].as_u64(){
        None => return None,
        Some(nanos) => nanos as u32,
    };
    Some(time::Duration::new(secs, nanos))
}

// TODO: pub too broad
/// Getting a string from a json
pub fn get_json_string(json: &serde_json::value::Value, index: &str) -> Option<String>{
    match json[index].as_str(){
        Some(json_str) => Some(String::from(json_str)),
        None => None,
    }
}
