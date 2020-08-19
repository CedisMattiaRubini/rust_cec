mod reciever;
mod sender;
mod error;
mod server;

const EXIT_COMMAND: &str = "exit-rust-cec";
const DEAFULT_SETTINGS_PATH: &str = "/etc/opt/cec_server/cec_settings.json";
const DEFAULT_LOG_PATH: &str = "/var/log/cec_server/cec-log";

pub use server::Server;
pub use reciever::traffic::responces::Response;
pub use reciever::traffic::responcesgroups::ResponseGroup;
pub use reciever::traffic::trigger::Trigger;
pub use error::{ServerError, Result};