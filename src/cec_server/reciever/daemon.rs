use std::sync::{Arc, Mutex, mpsc};
use std::process;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::process::Command;

use crate::log;
use crate::settings::Settings;
use super::traffic::Traffic;

#[derive(PartialEq)]
enum CodeCEC {
    In(String),
    Out(String),
    None,
}

/// Recieve the cec std output
/// Send all the output in a pipe that goes to the feeder
/// The feeder takes care of rerouting all the output to all those that request it
/// The feeder exist because it need to access a list of pipe in wich to send the output
/// This list is behind a lock
/// In order to not stop the reading operation of the stdout with each time there is a time to wait to get the lock, a separeta process (the feeder) takes care of that
pub fn launch(out_pipe: Arc<Mutex<Vec<mpsc::Sender<String>>>>, in_sender: mpsc::Sender<String>, cec_stdout: process::ChildStdout, mut log_setup: log::LogSetup, settings: Settings) {
    let reboot_flag = settings.can_reboot();
    let (sender, reciver) = mpsc::channel::<String>();
    let log_setup_clone = log_setup.clone();
    let mut traffic = Traffic::new(Some(settings), in_sender);
    thread::spawn(|| {
        send_out(out_pipe, reciver, log_setup_clone);
    });
    // Looping the stdout of the process
    for readed_result in BufReader::new(cec_stdout).lines() {
        // Did i readed corectly?
        match readed_result {
            Err(e) => {
                // When reading fails, log it
                log_setup.try_log(format!("Problem recieving from the cec daemon stdout, erro: {}", e));
            },
            Ok(readed) => {
                // Log the readings
                log_setup.try_log(format!("{}", readed));
                // Print the readings
                println!("{}", readed);
                // Sending to anyone who request the outputs
                if let Err(e) = sender.send(readed.clone()) {
                    log_setup.try_log(format!("Error while sending '{}'\n error {}", readed, e));
                };
                // Getting the actual traffic code, if its contained
                let wrapped_cec_code = get_cec_code(&readed, reboot_flag);
                if wrapped_cec_code != CodeCEC::None {
                    let cec_code = match wrapped_cec_code {
                        CodeCEC::In(code) => code,
                        CodeCEC::Out(code) => code,
                        CodeCEC::None => String::from("error"),
                    };
                    println!("-{}-", cec_code);
                    // get the response to the code
                    if let Err(e) = traffic.respond(&cec_code){
                        log_setup.try_log(format!("Could not responde to {}, error: {}", &cec_code, e));
                    };
                };
            },
        }
    }
}

/// Loops over the vec containing all the senders for each reciever listening to the CEC traffic
fn send_out(out_pipe: Arc<Mutex<Vec<mpsc::Sender<String>>>>, reciever: mpsc::Receiver<String>, mut log_setup: log::LogSetup) {
    // Waiting for something to send
    for message in reciever {
        // Getting the lock for the vec with all the sender
        match out_pipe.lock() {
            Err(e) => log_setup.try_log(format!("Error while sending to listenerd '{}'\n error {}", message, e)),
            // Sending to all sender
            // Ereasing dead pipes
            Ok(mut unlocked) => {
                // When sending, checks if a sender is still alive, if not, remove it from the vec
                let mut vec_to_remove = Vec::<usize>::new();
                for i in 0..unlocked.len() {
                    if let Err(_) = unlocked[i].send(message.clone()){
                        vec_to_remove.push(i);
                    };
                };
                for index in vec_to_remove{
                    unlocked.remove(index);
                };
            },
        };
    };
}

/// Filter the daemon output string and extract the CEC code
fn get_cec_code(readed: &String, reboot_flag: bool) -> CodeCEC {
    // If there was a problem with the cec daemon, reboot the pi
    if readed.contains("ERROR") && reboot_flag == true {
        reboot();
    };
    // It has to contain the TRAFFIC word at the beginning
    if readed.contains("TRAFFIC"){
        let split_at = if readed.contains("<<") {
            "<<"
        } else if readed.contains(">>") {
            ">>"
        } else {
            return CodeCEC::None
        };
        let splitted: Vec<&str> = readed.split(split_at).collect();
        if splitted.len() > 1 {
            let str_code: &str = &splitted[1][1..];
            if split_at == "<<" {
                CodeCEC::Out(String::from(str_code))
            } else {
                CodeCEC::In(String::from(str_code))
            }
        } else {
            CodeCEC::None
        }
    } else {
        CodeCEC::None
    }
}

/// Sometimes cec-daemon doesn't work and the raspberry pi has to be rebooted
fn reboot() {
    println!("Rebooting the rapsberry pi");
    thread::sleep(Duration::new(40, 0));
    loop {
        match  Command::new("reboot").spawn() {
            Err(e) => {
                println!("Error launching the shutdown: {}", e)
            },
            Ok(mut shutdown) => {
                match shutdown.wait() {
                    Err(e) => println!("Error shutting down: {}", e), 
                    Ok(exit_code) => println!("exit code: {}", exit_code),
                }
            },
        };
        thread::sleep(Duration::new(5, 0));
    }
}