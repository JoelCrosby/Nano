use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn read_stream_to_string(stream: &mut TcpStream) -> Result<String, io::Error> {
        // Read tream to buffer.
    let buffer = read_stream_to_buffer(&mut stream)?;
    // Read utf8 buffer to String.
    let result = String::from_utf8_lossy(&buffer).to_string();
    Ok(result)
}
// use crate::lib::res;
pub fn read_stream_to_buffer(stream: &mut TcpStream) -> Result<[u8; 1024], io::Error> {
    // Create a new empty buffer.
    let mut buffer = [0; 1024];
    // Fill buffer from stream.
    stream.read(&mut buffer)?;
    Ok(buffer)
}