/// The reciever feeds on the stdout of the cec server
/// A get_rsponse can be used to automatically respond with some command to some output

/// The reciever take care of reading the cec output get_rsponseing and sending it where it has to go

pub mod traffic;
pub mod daemon;

pub use daemon::launch;


