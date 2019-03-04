use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn res_ok(stream: &mut TcpStream, response: &str) -> Result<(), io::Error> {
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

pub fn res_not_found(stream: &mut TcpStream) -> Result<(), io::Error> {
    let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

pub fn res_internal_server_error(stream: &mut TcpStream, message: &str) -> Result<(), io::Error> {
    println!("{}", &message);
    let status_line = "HTTP/1.1 500 NOT FOUND\r\n\r\n";
    let response = format!("{}Content-Type: text/plain\r\n\r\n{}", status_line, message);
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

