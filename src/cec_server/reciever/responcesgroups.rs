use std::time;

use serde::{Serialize, Deserialize};

use super::responces::Response;

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseList (pub Vec<Response>);

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseGroup {
    pub(super) id: String,
    pub(super) responces: ResponseList,
    pub(super) delay_each: Option<time::Duration>,
    pub(super) delay_finish: Option<time::Duration>,
}

impl ResponseGroup {

    /// Convert a json in a ResponseGroup
    pub fn from_json(json: &serde_json::Value) -> super::Result<ResponseGroup> {
        // not checking the duration because it can be null
        if json["id"].is_string() && json["responces"].is_array() {
            // Conveting each element of the 
            let mut response_vec: Vec<Response> = Vec::new();
            if let Some(json_response_vec) = json["responces"].as_array() {
                for json_response in json_response_vec {
                    if let Ok(response) = Response::from_json(json_response) {
                        response_vec.push(response);
                    };
                };
            };
            let id = match super::get_json_string(&json, &"id"){
                Some(id) => id,
                None => return Err(super::ServerError::InvalidJson),
            };
            Ok(ResponseGroup {
                id: id,
                responces: ResponseList(response_vec),
                delay_each: super::json_duration(&json["delay_each"]),
                delay_finish: super::json_duration(&json["delay_finish"]),
            })
        } else {
            println!("Error 1");
            Err(super::ServerError::InvalidJson)
        }
    }

}

impl std::fmt::Display for ResponseGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let delay_each = match self.delay_each{
            Some(delay) => delay.as_secs(),
            None => 0,
        };
        let delay_finish = match self.delay_each{
            Some(delay) => delay.as_secs(),
            None => 0,
        };
        write!(f, "trigger: {}, delay_ech: {}, delay_finish: {}, responces: {}", self.id, delay_each, delay_finish, self.responces)
    }
}

impl std::fmt::Display for ResponseList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_string: String = String::new();
        for r in &self.0{
            print_string = format!("{}, {}", print_string, r);
        }
        write!(f, "{}", print_string)
    }
}