use std::thread;
use std::sync::mpsc;

use super::{Error, daemon, KILL, TEST,};

/// Used to manage the log server
pub(super) struct MasterLog {
    /// Handle used to check if the logger thread is running
    /// Also used to check if it's running
    log_handle: Option<thread::JoinHandle<()>>,
    /// Pipe used to communicate with the logger thread
    /// Directly used by LogStetup
    in_pipe: Option<mpsc::Sender<String>>,
}

impl MasterLog {

    /// Create a new thread that logs messages
    /// Also initialize all the needed tools
    pub(super) fn new(log_file: String) -> super::Result<MasterLog> {
        // Creating/opening the log file
        let f = match std::fs::OpenOptions::new().create(true).append(true).open(log_file){
            Err(e) => return Err(Error::CouldNotOpenLogFile(e)),
            Ok(f) => f,
        };
        // Creating the pipes used by the log system
        let (in_pipe, out_pipe): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        // Creating the thread that log everything
        let handle = thread::spawn(move || {
            daemon::log_loop(f, out_pipe)
        });
        // Everthing is working, returning a log setup
        Ok(
            MasterLog {
                log_handle: Some(handle),
                in_pipe: Some(in_pipe),
            }
        )
    }

    pub(super) fn empty(& mut self) {
        self.log_handle = None;
        self.in_pipe = None;
    }

    /// Creates a LogSetup from the LogMaster that generates it
    pub(super) fn new_setup(& mut self, source: String) -> super::Result<(String, Option<mpsc::Sender<String>>)> {
        // Checking if there still is a pipe
        match self.in_pipe {
            None => return Err(Error::DeadDaemon),
            Some(ref pipe) => {
                // Send a test string to check is the cosumer has been dropped
                match pipe.send(String::from(TEST)) {
                    Ok(()) => pipe.clone(),
                    // If sending the log fails, the log is to be considered dead
                    Err(_) => {
                        self.in_pipe = None;
                        return Err(Error::DeadDaemon)
                    },
                }
            },
        };
        // Everythng is ok
        Ok(
            (source, self.in_pipe.clone())
        )
    }

    /// Check if the the log thread is alive
    pub(super) fn is_alive(& self) -> bool {
        match self.in_pipe {
            None => false,
            Some(ref pipe) => {
                match pipe.send(String::from(TEST)) {
                    Err(_) => false,
                    Ok(_) => true,
                 }
            },
        }
    }

    /// Kill the logging process in order to not leave hanging threads or to start a new istance
    /// In order to function the pipe must be working
    pub(super) fn kill(& mut self) -> super::Result<()> {
        // Get pipe if it still exists
        match self.in_pipe {
            None => return Err(Error::DeadDaemon),
            // Send the kill command
            Some(ref pipe) => {
                match pipe.send(String::from(KILL)) {
                    // The thread is dead
                    Err(_) => return Err(Error::DeadDaemon),
                    // Kill command sent
                    Ok(()) => {
                        // Waiting for the thread to die
                        if let Some(handle) = self.log_handle.take() {
                            if let Err(_) = handle.join(){
                                return Err(Error::DeadDaemon)
                            };
                        };
                        return Ok(())
                    },
                }
            },
        }
    }

}// impl MasterLog