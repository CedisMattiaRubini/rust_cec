use std::marker::Unpin;
use std::borrow::Cow;
use futures::io::{AsyncRead, AsyncReadExt};

use super::error::{Result, Error};

pub async fn receive_json<T: AsyncRead + Unpin>(stream: & mut T) -> Result<serde_json::Value>{
    // Reading the json
    let mut json_bits: Vec<u8> = recieve_exact(stream).await?;
    // Converting the readed json in a string json
    let json_str: Cow<str> = String::from_utf8_lossy(json_bits.as_mut_slice());
    match serde_json::from_str(&json_str) {
        Err(e) => Err(Error::SerdeJsonError(e)),
        Ok(json) => Ok(json), 
    }
}

 pub async fn recieve_exact<T: AsyncRead + Unpin>(stream: & mut T) -> Result<Vec<u8>> {
    // Creating the buffer to read the size of the data
    let buffer: & mut [u8; 4] = & mut [0; 4];
    // Reading the metadata lenght: u32 in big-endian
    stream.read_exact(buffer).await?;
    // Converting the data lenght
    let len: usize = u32::from_be_bytes(*buffer) as usize;
    // Array lenght cannot be created at compile time
    // Create a vec (dyn array) and then use it as an array
    let mut data: Vec<u8> = vec![0; len];
    // From the vec get the buffer of custom lenght
    let buffer: & mut [u8] = data.as_mut_slice();
    // reading the data
    stream.read_exact(buffer).await?;
    Ok(data)
}