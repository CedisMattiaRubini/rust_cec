use serde_json::json;

use std::net::TcpStream;

use crate::settings::Settings;
use crate::communication;
use communication::synchronous::{receive, send};

/// Recieve new settings or give back the one used right now
pub fn settings_handler(stream: TcpStream, json: serde_json::Value, settings: Settings) {
    if let serde_json::Value::String(conn_type) = &json["command"]{
        match conn_type.as_str() {
            "get" => send_settings(stream, settings),
            "set" => recieve_settings(stream, settings),
            // TODO:
            _ => println!("invalid command"),
        }
    } else {
        // TODO:
        println!("Missing command");
    };
    
}

// Return settings to the stream
fn send_settings(mut stream: TcpStream, settings: Settings) {
    if let Some(settings) = settings.read_settings(){
        // TODO:
        send::send_json(&mut stream, &settings);
    } else {
        // TODO:
        println!("Unable to retieve the settings");
    };
}

// Get new settings from the stream and return the new saved settings
fn recieve_settings(mut stream: TcpStream, settings: Settings) {
    if let Ok(json) = receive::receive_json(&mut stream){
        settings.save_settings(json);
        let readed_settings = settings.retrieve_responces();
        // Get new settings and return to sender
        let json_response = match readed_settings {
            //
            Some(s) => s,
            // Send empty json
            None => json!({}),
        };
        // Sending back new settings
        // TODO:
        send::send_json(&mut stream, &json_response);
    } else {
        // TODO:
        println!("Unable to receive settings json");
    }
}