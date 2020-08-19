use serde::{Serialize, Deserialize};

use std::time;
use std::thread;

use super::responces::Response;
use super::jsonparser;

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseGroup {
    /// Identifier
    pub(super) id: String,
    /// Responses
    pub(super) responces: Vec<Response>,
    /// Delay before
    pub(super) delay_each: Option<time::Duration>,
    /// Delay after
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
            let id = match jsonparser::get_json_string(&json, &"id"){
                Some(id) => id,
                None => return Err(super::ServerError::InvalidJson),
            };
            Ok(ResponseGroup {
                id: id,
                responces: response_vec,
                delay_each: jsonparser::json_duration(&json["delay_each"]),
                delay_finish: jsonparser::json_duration(&json["delay_finish"]),
            })
        } else {
            println!("Error 1");
            Err(super::ServerError::InvalidJson)
        }
    }

    /// Return true if the id are equals
    pub fn eq_id(&self, id: &String) -> bool{
        self.id.eq(id)
    }

    /// Execute the group delauy
    pub fn delay(&self) -> bool {
        if let Some(delay) = self.delay_each {
            thread::sleep(delay);
            return true
        };
        false
    }

    /// Execute the group final delay
    pub fn final_delay(&self) -> bool {
        if let Some(delay) = self.delay_finish {
            thread::sleep(delay);
            return true
        };
        false
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
        let mut print_string: String = String::new();
        for r in &self.responces{
            print_string = format!("{}, {}", print_string, r);
        };
        write!(f, "trigger: {}, delay_ech: {}, delay_finish: {}, responces: {}", self.id, delay_each, delay_finish, print_string)
    }
}

impl <'a> IntoIterator for & 'a ResponseGroup {

    type Item = <std::slice::Iter<'a, Response> as Iterator>::Item;
    type IntoIter = std::slice::Iter<'a, Response>;

    fn into_iter(self) -> Self::IntoIter {
        self.responces.as_slice().into_iter()
    }

}