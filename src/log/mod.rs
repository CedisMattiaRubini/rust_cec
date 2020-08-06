/// The log consist in a daemon inside a dedicated thread that recieve via a pipe the content to log
/// All the communicatione and thread management are managed by this module and submodules

pub use server::LogServer;
pub use setup::LogSetup;
pub use error::{Error, Result};

use master::MasterLog;

pub mod error;
pub mod setup;
pub mod server;
pub mod master;

mod daemon;

// Kill command, stop the log thread
const KILL: &str = "kill_log";
// Test the pipe to check if the thread has been terminated
const TEST: &str = "test_str";

// TODO: gracefull shutdown