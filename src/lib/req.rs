use std::io;
use std::net::TcpStream;

use crate::lib::stream;

pub struct Request {
    pub buffer: [u8; 1024],
    pub raw: String,
    pub path: String,
}

pub fn read_stream_to_request(tcp_stream: &mut TcpStream) -> Result<Request, io::Error> {

    let buffer = stream::read_stream_to_buffer(tcp_stream)?;
    let strget = String::from_utf8_lossy(&buffer).to_string();

    let chunks: Vec<&str> = strget.split_whitespace().collect();

    let resource_path = match find_resource_path_in_stream(chunks) {
        Some(val) => val,
        None => "",
    };

    let result = Request {
        buffer: buffer,
        raw: strget,
        path: resource_path.to_string()
    };

    Ok(result)
}

fn find_resource_path_in_stream(chunks: Vec<&str>) -> Option<&str> {
    let start = "/";
    for chunk in chunks.iter() {
        if chunk.starts_with(start) {
            return Some(chunk);
        }
    }
    None
}