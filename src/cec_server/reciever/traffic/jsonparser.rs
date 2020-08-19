use std::time;

/// check if the json is compatible with time::duration
fn is_duration(json: &serde_json::value::Value) -> bool {
    !json.is_null() && json["nanos"].is_number() && json["secs"].is_number()
}

/// convert the json (if compatible) with time::duration
pub fn json_duration(json: &serde_json::value::Value) -> Option<time::Duration>{
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