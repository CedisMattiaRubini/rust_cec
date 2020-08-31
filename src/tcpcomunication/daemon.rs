use std::net::{TcpListener, SocketAddr, IpAddr};
use std::sync::{mpsc, Arc, Mutex};

use crate::communication::synchronous::receive;
use crate::settings::Settings;

enum Request {
    Outuput(mpsc::Sender<String>),
    Settings,
    None
}

/// Deling with incoming tcp connection
pub fn tcp_handler(ip: IpAddr, port: u16, sender: mpsc::Sender<String>, reciever: Arc<Mutex<Vec<mpsc::Sender<String>>>>, settings: Settings) -> std::io::Result<()>{
    let socket = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(socket)?;
    for raw_stream in listener.incoming() {
        println!("new stream");
        let (new_sender, new_reciever) = mpsc::channel();
        // Getting the mutex guard
        let mut data_vec = match reciever.lock(){
            Ok(data) => data,
            Err(e) => {
                // TODO: better handling of errors
                println!("Comunication failed, error: {}", e);
                continue
            },
        };
        (*data_vec).push(new_sender);
        // TODO: new thread for each new open connection
        match raw_stream {
            Err(e) => {
                // TODO: better handling of errors
                println!("Comunication failed, error: {}", e);
                continue
            },
            Ok(mut stream) => {
                // Get a json containing what the client wants
                let rjson = receive::receive_json(&mut stream);
                match rjson {
                    // TODO:
                    Err(_e) => println!("wrong json"),
                    Ok(json) => {
                        // Checking if the json has the request field and it's a string
                        if let serde_json::Value::String(conn_type) = &json["request"]{
                            // Finding out wich request it is
                            let request = match conn_type.as_str(){
                                "output" => Request::Outuput(sender.clone()),
                                "settings" => Request::Settings,
                                _ => Request::None,

                            };
                            // Responding accordingly to each request
                            match request{
                                Request::Outuput(sender) => {
                                    std::thread::spawn(move || {
                                        super::cec_traffic::output_handler(stream, sender, new_reciever);
                                    });
                                },  
                                Request::Settings => {
                                    let cloned_settings = settings.clone();
                                    std::thread::spawn(move || {
                                        super::settings::settings_handler(stream, json, cloned_settings);
                                    });
                                },
                                Request::None => println!("invalid request"),
                            }
                        } else {
                            // TODO
                            println!("Wrong request");
                        }
                    },
                };
            },
        };
    };
    Ok(())
}

