/// Manages the module that manages the traffic in the CEC
use crate::cec_server::{Result, ServerError};
use crate::settings::Settings;

use std::sync::mpsc;

pub mod responces;
pub mod responcesgroups;
pub mod trigger;
pub mod jsonparser;
pub mod error;

pub struct Traffic {
    trigger: Option<Vec<trigger::Trigger>>,
    responcesgroups: Option<Vec<responcesgroups::ResponseGroup>>,
    in_sender: mpsc::Sender<String>,
}

impl Traffic{

    /// Creates new traffic settings
    pub fn new(source: Option<Settings>, in_sender: mpsc::Sender<String>) -> Traffic{
        // Read from settings if provided
         if let Some(file_settings) = source {
            if let Some((trigger, responcesgroups)) = Traffic::get_from_json(file_settings){
                return Traffic {
                    trigger: Some(trigger),
                    responcesgroups: Some(responcesgroups),
                    in_sender: in_sender,
                }
            };
         };
         // No settings or no setting file
         Traffic {
             trigger: None,
             responcesgroups: None,
             in_sender: in_sender,
         }
    }

    pub fn get_from_json(settings: Settings) -> Option<(Vec<trigger::Trigger>, Vec<responcesgroups::ResponseGroup>)> {
        if let Some((responcesgroups, trigger)) = settings.retrieve_responces(){
            return Some((trigger, responcesgroups));
        };
        None
    }

    /// Return wether the trigger is present or not
    pub fn has_triggered(&self, data: &String) -> Option<&String> {
        // Does it has trigger
        if let Some(some_trigger) = &self.trigger {
            // Check if the trigger exists over the whole list
            for t in some_trigger {
                if let Some(response) = t.has_trigger(data) {
                    return Some(response)
                };
            };
        };
        return None
    }

    pub fn get_response(&self, id: &String) -> Option<&responcesgroups::ResponseGroup> {
        if let Some(group) = &self.responcesgroups {
            for resp_group in group {
                if resp_group.eq_id(id) {
                    return Some(&resp_group)
                };
            };
        };
        None
    }

    /// Respond to a code if there is any trigger
    pub fn respond(& mut self, data: &String) -> error::Result<()> {
        // Get the response id if there is one
        if let Some(response_id) = self.has_triggered(data) {
            // Get the group response
            if let Some(resp_group) = self.get_response(response_id){
                // Getting single response
                for resp in resp_group {
                    // Delay resp > Delay group
                    if !resp.delay(){
                        resp_group.delay();
                    };
                    // Sending single code
                    if let Err(e) = self.in_sender.send(resp.get_response().clone()){
                        return Err(error::TrafficError::CouldNotSendResponse(e))
                    };
                }
                // Final delay
                resp_group.final_delay();
                return Ok(())
            };
        };
        return Err(error::TrafficError::NoResponse)
    }

}