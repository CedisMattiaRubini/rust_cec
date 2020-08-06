/// The daemon logic: format and logs all the messages that it recieve

use std::io::Write;
use std::sync::mpsc;

extern crate chrono;

use super::{KILL, TEST};

/// Log a string
pub fn new_entry(mut log_file: &std::fs::File, log: String) {
    let now = chrono::prelude::Utc::now();
    let full_log = String::from(format!("[UTC {}]   {}", now, log));
    if let Err(e) = log_file.write_all(full_log.as_bytes()){
        println!("Error while logging {}\nError: {}", full_log, e)
    }
}

/// Loop that takes all the logs and logs them
/// The file where to log, and the pipe from wich the logs flow is given
pub fn log_loop(log_file: std::fs::File, out_log: mpsc::Receiver<String>) {
    for log in out_log {
        let compare: &str = &log;
        match compare {
            KILL => break,
            TEST => (),
            _ => new_entry(&log_file, log),
        }
    } 
}