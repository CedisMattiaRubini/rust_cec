/// Before using those modules there is a consideration to be made
/// Shoud there be a global varible containing all the settings, or not?
/// Tecnically yes, but practically: one person will setup the device and leave there 
/// Even if the settings have to be reworked, this cannot cause massive problems
/// The global variable would introcude problems that require time to solve, for situation that are very uncommon
/// There are no controls because the client is expected to take care of those problem

use std::fs;
use std::io::{Read, Write};
use std::path;

use serde_json;

use crate::cec_server;

pub struct Settings{
    settings_path: String,
}

impl Clone for Settings {
    fn clone(&self) -> Self {
        Settings{
            settings_path: self.settings_path.clone(),
        }
    }
}

impl Settings {

    pub fn new(file_path: String) -> Settings {
        Settings {
            settings_path: file_path
        }
    }

    /// Recieve a json and save it in settings
    pub fn save_settings(&self, json: serde_json::Value) {
        // TODO: 
        fs::remove_file(& self.settings_path);
        let mut js_settings_file: fs::File = match fs::OpenOptions::new().read(true).write(true).create(true).open(path::Path::new(& self.settings_path)){
            Err(e) => panic!("Couldn't open/create file, error: {}",e),
            Ok(js_settings) => js_settings,
        };
        // TODO
        if let Err(e) = js_settings_file.write_fmt(format_args!("{}", json)){
            println!("{}", e);
        } else {
            println!("Success writing");
        };
    }

    /// Read the json settings file and return a json
    pub fn read_settings(&self) -> Option<serde_json::value::Value> {
        // Opening the file
        let mut js_settings_file: fs::File = match fs::OpenOptions::new().read(true).write(true).create(true).open(path::Path::new(& self.settings_path)){
            Err(e) => panic!("Couldn't open/create file, error: {}",e),
            Ok(js_settings) => js_settings,
        };
        // Reading the content and placing it in a String
        let mut js_settings_string: String = String::new();
        match js_settings_file.read_to_string(& mut js_settings_string){
            Ok(readed) => {
                if readed == 0 {
                    return None
                };
            },
            Err(e) => panic!("{}",e),
        };
        // Checking the readed content
        let json: serde_json::value::Value = match serde_json::from_str::<serde_json::value::Value>(js_settings_string.as_str()){
            Ok(json_str) => json_str,
            // TODO:
            Err(e) => return None,
        };
        Some(json)
    }

    /// From the settings retrieve the responces
    pub fn retrieve_responces(&self) -> Option<(Vec<cec_server::ResponseGroup>, Vec<cec_server::Trigger>)> {
        // Get settings from the json file
        let json_settings: serde_json::value::Value = match self.read_settings() {
            None => return None,
            Some(json) => json,
        };
        // Getting triggers
        let triggers_array: Vec<cec_server::Trigger> = match json_settings["responces"]["triggers"].is_array() {
            false => return None,
            true => match json_settings["responces"]["triggers"].as_array(){
                None => return None,
                Some(array) => {
                    let mut trigger_vec: Vec<cec_server::Trigger> = Vec::new();
                    for json_trigger in array {
                        if let Ok(trigger) = cec_server::Trigger::from_json(json_trigger){
                            trigger_vec.push(trigger);
                        };      
                    };
                    trigger_vec
                }, 
            },
        };
        // Getting responces
        let responces_array: Vec<cec_server::ResponseGroup> = match json_settings["responces"]["responces_groups"].is_array(){
            false => return None,
            true => match json_settings["responces"]["responces_groups"].as_array() {
                None => return None,
                Some(array) => {
                    let mut group_responces_vec: Vec<cec_server::ResponseGroup> = Vec::new();
                    for json_responce_group in array {
                        if let Ok(group_responce) = cec_server::ResponseGroup::from_json(json_responce_group){
                            group_responces_vec.push(group_responce);
                        };
                    };
                    group_responces_vec
                },
            },
        };
        // Checking if there are responces
        Some((responces_array, triggers_array))
    }

    /// Flag 
    pub fn can_reboot(& self) -> bool {
        let json_settings = match self.read_settings(){
            Some(json) => json,
            None => return false,
        };
        if json_settings["reboot"].is_boolean() {
            match json_settings["reboot"].as_bool() {
                Some(flag) => flag,
                None => false,
            }
        } else {
            false
        }
    }

}


/* 

http://www.cec-o-matic.com/

EXAMPLE OF JSON

{
    "responces": {

        "triggers": [
            {
                "trigger": "String",
                "response_id": "String"
            }
        ],

        "responces_groups": [
            {
                "id": "String",
                "delay_each": {
                    "secs": 0,
                    "nanos":0
                },
                "delay_finish": null,
                "responces": [
                    {
                        "cmd": "String",
                        "delay": {
                            "secs": 0,
                            "nanos":0
                        }
                    }
                ]
            }
        ]

    }
}
*/