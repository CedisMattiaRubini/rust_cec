use std::net::TcpStream;
use std::sync::mpsc;

use crate::communication;
use communication::synchronous::{receive, send};

/// Relay the output of the cec daemon and recieve command to pass at the daemon
/// Use threads
pub fn output_handler(mut stream: TcpStream, sender: mpsc::Sender<String>, reciever: mpsc::Receiver<String>) {
    // Setting the timeout of the connection to 0 to avoid dropout
    // If the connection is close, the read write will yield an error
    stream.set_read_timeout(None);
    // TODO: carefull cloning streams
    if let Ok(mut stream_clone) = stream.try_clone() {
        // Sending the output in a separated thread
        std::thread::spawn(move || {
            for message in reciever {
                let json = serde_json::json!({
                    "type":"traffic",
                    "message":message,
                });
                // TODO:
                if let Err(e) = send::send_json(&mut stream, &json){
                    println!("{}",e);
                    if let communication::Error::TcpError(_) = e{
                        break;
                    };
                };
            };
        });
        // Recieving commands in a separated thread
        std::thread::spawn(move || {
            loop {
                let rjson = receive::receive_json(&mut stream_clone);
                match rjson {
                    // TODO:
                    Err(e) => {
                        println!("{}", e);
                        if let communication::Error::TcpError(_) = e{
                            break;
                        };
                    },
                    Ok(json) => {
                        if let serde_json::Value::String(command) = &json["tx"] {
                            let cmd = format!("tx {}", command);
                            //TODO:
                            sender.send(cmd);
                        } else {
                            // TODO: 
                            println!("Wrong command");
                        };
                    },
                };
            };
        });
    }; 
}   

// Read a message in the tcp stream