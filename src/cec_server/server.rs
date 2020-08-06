/// The CEC server spawn and manage 2 threads: sender and reciever
/// The reciever will consume all the stdout of the cec server
/// The sender will feed cec inputs to the cec server stdin
/// The server manages the in/out thread and all communication with those

use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::process::{Command, Stdio};

use crate::log;
use crate::settings::Settings;

use super::error;
use super::sender;
use super::reciever;
use super::DEAFULT_SETTINGS_PATH;
use super::DEFAULT_LOG_PATH;

/// The server stucture contains all the pharameter needed
/// Te server has a main structure and several thread
/// The sender thread takes care to input cec codes
/// The reciever reads the cec traffict and can react in response to some code
pub struct Server{
    /// the process running the cec deamon
    cec_daemon: std::process::Child,
    /// Settings configuation
    settings: Settings,
    /// Master used to create SetupLog used to log
    log_server: log::LogServer,
    /// The handle to the sender thread
    sender_handle: thread::JoinHandle<()>,
    /// The sender pipe to send messages to the sender
    in_pipe_sender: mpsc::Sender<String>,
    /// The handle to the reciever thread
    reciever_handle: thread::JoinHandle<()>,
    /// The reciever pipe to recieve the cec output
    /// Since mpsc are multiple producer SINGLE consumer, there is the need of a vec, each element is created for a consumer
    /// When a process has to recieve the stdout the server will create a new set of pipes
    /// The reciever is given to the process and the sender is added to this list
    in_pipe_reciever: Arc<Mutex<Vec<mpsc::Sender<String>>>>,
}

impl Server{

    /// Creates and initialize a new istance of the server and it's threads
    pub fn new(log_settings: Option<log::LogServer>, settings_settings: Option<Settings>) -> error::Result<Server> {
        // 1) Launch the cec library

        let cec_spawn_result = Command::new("cec-client")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn();
        let mut cec_daemon = match cec_spawn_result {
            Err(e) => return Err(error::ServerError::CecDaemonNotSpawned(e)),
            Ok(spawn) => spawn,
        };
        
        //2) Creating the log thread
        // If it wasn't specified, use the default setting
        let log_server = match log_settings {
            Some(path) => path,
            None => {
                match log::LogServer::new(DEFAULT_LOG_PATH.to_string()){
                    Err(e) => return Err(error::ServerError::LogError(e)),
                    Ok(log) => log,
                }
            },
        };
        
        // Creating the settings struct
        // If not specified, use the default setting
        let settings = match settings_settings {
            Some(path) => path,
            None => {
                Settings::new(DEAFULT_SETTINGS_PATH.to_string())
            },
        };

        // 3) Create fisrt the sender and than the reciever thread
        // The sender has to be created first because the reciever needs a pipe to communicate with the seder
        // The reciver needs a pipe to communicate with the sender therefore the sender needs to be created first
        
        // Getting the cec daemon stdin
        let cec_stdin = match cec_daemon.stdin.take() {
            None => {
                match cec_daemon.kill(){
                    Err(kill_e) => return Err(error::ServerError::CecDaemonNotKilled(kill_e, Box::new(Some(error::ServerError::CecStdInMissing)))),
                    Ok(()) => return Err(error::ServerError::CecStdInMissing),
                };
            },
            Some(stdin) => stdin,
        };
        // Creating the clones to send to the thread (ownership compliance)
        let sender_log = match log_server.new_setup(String::from("rust_cec_server_sender")) {
            Err(e) => return Err(error::ServerError::LogError(e)),
            Ok(log) => log,
        };
        // Init the sender and its pipes
        let (in_sender, out_sender): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        let sender_handle = thread::spawn(move || {
            sender::launch(out_sender, cec_stdin, sender_log)
        });
        // Getting the cec daemon stdout
        let cec_stdout = match cec_daemon.stdout.take() {
            None => {
                match cec_daemon.kill(){
                    Err(kill_e) => return Err(error::ServerError::CecDaemonNotKilled(kill_e, Box::new(Some(error::ServerError::CecStdOutMissing)))),
                    Ok(()) => return Err(error::ServerError::CecStdOutMissing)
                };   
            },
            Some(stdout) => stdout,
        };
        // Creating the clones to send to the thread (ownership compliance)
        let in_sender_clone = in_sender.clone();
        let reciever_log = match log_server.new_setup(String::from("rust_cec_server_reciever")){
            Err(e) => return Err(error::ServerError::LogError(e)),
            Ok(log) => log,
        };
        // Init the vec of ech pipe
        let in_reciever: Arc<Mutex<Vec<mpsc::Sender<String>>>> = Arc::new(Mutex::new(Vec::new()));
        let in_reciever_clone = in_reciever.clone();
        let cloned_settings = settings.clone();
        // Init the reciever
        let reciever_handle = thread::spawn(move || {
            reciever::launch(in_reciever_clone, in_sender_clone, cec_stdout, reciever_log, cloned_settings)
        });
        // TODO: take care of the stderr

        // The server is ready
        Ok(
            Server{
                cec_daemon: cec_daemon,
                settings: settings,
                log_server: log_server,
                sender_handle: sender_handle,
                in_pipe_sender: in_sender,
                reciever_handle: reciever_handle,
                in_pipe_reciever: in_reciever,
            }
        )
    } // new

    // TODO: add may other needed things
    /// Gracefully shutdown everything
    pub fn kill(& mut self) -> std::io::Result<()> {
        self.cec_daemon.kill()
    }

    /// Giving out a channel to send input to the cec process
    pub fn new_sender(&mut self) -> mpsc::Sender<String>{
        self.in_pipe_sender.clone()
    }

    pub fn get_reciever_vec(&mut self) -> Arc<Mutex<Vec<mpsc::Sender<String>>>>{
        self.in_pipe_reciever.clone()
    } 

}// impl Server