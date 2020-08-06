// Defining mods
pub mod receive;
pub mod send;

// Giving access to modules
use super::error;

// External crates
use serde::{Deserialize, Serialize};

// Uses
pub use error::{Result, Error};

/// Data describing what is coming! such as winter.
#[derive(Serialize, Deserialize)]
struct MetaData {
    //size: i64,
    content_type: String,
}