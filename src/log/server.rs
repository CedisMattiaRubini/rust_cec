use std::sync::{Arc, Mutex, MutexGuard};

use super::{Error, MasterLog, LogSetup};

/// This tuple struct exist to wrap the MasterLog in a mutex to be used across threads
#[derive(Clone)]
pub struct LogServer {
    master: Arc<Mutex<MasterLog>>,
    log_path: String,
}

impl LogServer {

    // place empty where it has to be placed

    /// Create a new istance of the tuple struct
    pub fn new(log_file: String) -> super::Result<LogServer> {
        let master_log = MasterLog::new(log_file.clone())?;
        Ok(
            LogServer{
                master: Arc::new(Mutex::new(master_log)),
                log_path: log_file,
            } 
        )
    }

    /// If the log thread die, use this method to regenerate it
    pub fn regenerate(& mut self) -> super::Result<()> {
        *self = LogServer::new(self.log_path.clone())?;
        Ok(())
    }

    /// This is a veryrepetitive operation, enche the function
    /// Also makes all other function that use it a lot more readable
    fn unlock_master_log(& self) -> super::Result<MutexGuard<MasterLog>> {
        match self.master.lock() {
            Err(_) => Err(Error::DeadMaster),
            Ok(lock_master_log) => Ok(lock_master_log),
        }
    }

    pub fn empty(& self) -> super::Result<()> {
        self.unlock_master_log()?.empty();
        Ok(())
    }

    /// Creates a LogSetup from the LogMaster that generates it
    /// Blocking function
    pub fn new_setup(& self, source: String) -> super::Result<LogSetup> {
        let mut loc_master_log = self.unlock_master_log()?;
        let (source, in_pipe) = loc_master_log.new_setup(source)?;
        Ok(
            LogSetup {
                source: source,
                in_pipe: in_pipe,
                master: (*self).clone()
            }
        )
    }

    /// Checks if the daemon is alive
    pub fn is_alive(& self) -> super::Result<bool> {
        let lock_master_log = self.unlock_master_log()?;
        Ok(lock_master_log.is_alive())
    }

    // Kills the daemon
    pub fn kill(& self) -> super::Result<()> {
        let mut lock_master_log = self.unlock_master_log()?;
        lock_master_log.kill()
    }

}// impl LogServer