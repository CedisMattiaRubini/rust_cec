/// The senders is a single thread that redirect all messages to the stdin

use std::sync::mpsc;
use std::process;
use std::io::Write;

use crate::log;

use super::EXIT_COMMAND;

/// Send to the cec std input
pub(super) fn launch(in_pipe: mpsc::Receiver<String>, mut cec_stdin: process::ChildStdin, mut log_setup: log::LogSetup) {
    // Wait for a new message to relay
    for recieved in in_pipe {
        log_setup.try_log(format!("Sent: {}", recieved));
        if recieved == String::from(EXIT_COMMAND){
            break
        };
        let message: String = format!("{}\r\n", recieved);
        if let Err(e) = cec_stdin.write_all(message.as_bytes()) {
            log_setup.try_log(format!("Error sending command: {}\nError: {}", recieved, e));
        };
    };
}

// TODO: Check if the string is either a cec code, or valid command for the tool
//pub fn is_valid(cec_code: String) -> bool{}