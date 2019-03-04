use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn read_stream_to_buffer(stream: &mut TcpStream) -> Result<[u8; 1024], io::Error> {
    // Create a new empty buffer.
    let mut buffer = [0; 1024];
    // Fill buffer from stream.
    stream.read(&mut buffer)?;
    Ok(buffer)
}