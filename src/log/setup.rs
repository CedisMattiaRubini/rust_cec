use std::sync::mpsc;

use super::{Error, TEST, LogServer};
/// Created to streamline the loggig action
/// It's a child spawn from a MasterLog
/// This struct are ment to be given one per thread
#[derive(Clone)]
pub struct LogSetup {
    /// Help pintpoint where the problem came
    pub(super) source: String,
    /// Used to communicate with the log thread, to log
    pub(super) in_pipe: Option<mpsc::Sender<String>>,
    // Contains a link to the master that generated it
    // It's used to regenerate a new thread if needed
    // Since it can be sent to variuos thread, and it's possible to have a race condition
    // In order to avoid the race, a lock is used
    pub(super) master: LogServer,
}

impl LogSetup {

    /// Empty all values
    pub fn empty(& mut self) {
        self.in_pipe = None;
    }

    /// If the log daemon is dead, it's possible to regenarate it
    pub fn regenerate(& mut self) -> super::Result<()> {
        self.master.regenerate()
    }

    /// Get the pipe of the log setup, if ther ins't a pipe the daemon is dead
    fn get_pipe(& self) -> super::Result<mpsc::Sender<String>> {
        match self.in_pipe {
            None => Err(Error::DeadDaemon),
            Some(ref pipe) => Ok(pipe.clone()),
        }
    }

    /// Checks if the log thread is alive
    pub fn is_alive(& mut self) -> bool {
        // if there is no pipe, the daemon is dead
        match self.get_pipe() {
            Err(_) => false,
            Ok(pipe) => {
                // sending a test string to check if the daemon still exists
                match pipe.send(String::from(TEST)) {
                    Ok(_) => true,
                    Err(_) => {
                        // if the pipe dosn't work, init the struct
                        self.empty();
                        false
                    },
                }
            },
        }
    }

    /// Log a String
    pub fn log(& mut self, message: String) -> super::Result<()> {
        // Format the message
        let log = format!("[{}]   [{}]\n", self.source, message);
        // The formattet message is sent to the logger daemon
        let pipe = self.get_pipe()?;
        match pipe.send(log) {
            // The log thread is dead
            Err(_) => {
                self.empty();
                Err(Error::DeadDaemon)
            },
            Ok(()) => Ok(()),
        }
    }

    /// Log if possible and is non blocking
    pub fn try_log(& mut self, message: String) {
        if let Err(e) = self.log(message) {
            match e {
                Error::DeadDaemon | Error::CouldNotOpenLogFile(_) => {
                    
                },
                Error::DeadMaster => (),
            };
        }
    }

}// impl LogSetup