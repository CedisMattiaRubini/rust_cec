// Std
use std::io::Write;

// Not std
use super::error::{Result, Error};

/// Send a json as a UTF-8 string
pub fn send_json<T: Write>(sender: & mut T, json: & serde_json::Value) -> Result<()> {
    // Serializing the json in a str UTF-8
    let json_str: String = json.to_string(); 
    // From the json in UTF-8 get each individual bits in a slice
    let json_bits: & [u8] = json_str.as_bytes();
    send_exact(sender, json_bits)?;
    Ok(())
}

/// Sends the data feeded to it
/// Also in charge of encrypting the data
pub fn send_exact<T: Write>(sender: & mut T, data: & [u8]) -> Result<()>{
    // Getting the data lenght and checking isn't too much
    let data_len: u32 = if (*data).len() <= (std::u32::MAX as usize) {
        (*data).len() as u32
    } else {
        return Err(Error::DataExcess)
    };
    sender.write(&data_len.to_be_bytes())?;
    sender.write(data)?;
    Ok(())
}