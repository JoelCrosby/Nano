use std::io;
use std::net::TcpStream;

use crate::lib::stream;

struct Request {
    raw: String,
    path: String,
}

pub fn read_stream_to_request(stream: &mut TcpStream) -> Result<Request, io::Error> {

    let strget = stream::read_stream_to_string(&mut stream)?;

    let chunks: Vec<&str> = strget.split_whitespace().collect();

    let resource_path = match find_resource_path_in_stream(chunks) {
        Some(val) => val,
        None => "",
    };

    let result = Request {
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