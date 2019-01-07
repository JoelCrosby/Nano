extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::net::TcpListener;
use std::net::TcpStream;

#[derive(Serialize, Deserialize)]
struct Options {
    wwwroot: String,
    address: String,
}

fn main() {
    
    let o = load_configuration().unwrap();

    println!("\nNano server started!");
    println!("\nBinding address... {}", o.address);

    let listener = TcpListener::bind(o.address);

    let listener = match listener {
        Ok(socket) => socket,
        Err(error) => {
            println!("\nNano server was unable to start {}", error);
            return;
        },
    };

    println!("\nListening for connections...\n");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &o.wwwroot);
    }
}

fn load_configuration() -> Result<Options, io::Error>  {
    let path = "nano.json".to_string();
    let config = read_file(&path).unwrap();

    let v: Options = serde_json::from_str(&config)?;

    Ok(v)
}

fn handle_connection(mut stream: TcpStream, wwwroot: &String) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let strget = String::from_utf8_lossy(&buffer);
    

    let chunks:Vec<_> = strget.split_whitespace().collect();
    let getfile = chunks[1].replace("/", "\\")
        .trim_left_matches("\\").to_string();


    let mainpage = "index.html".to_string();

    println!("Path: {}", &getfile);

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", &mainpage)
    } else {
        ("HTTP/1.1 200 OK\r\n\r\n", &getfile)
    };

    let mut w = wwwroot.clone();

    w.push_str(&filename);
    let contents = read_file(&w);

    let res = match contents {
        Ok(c) => Ok(c),
        Err(e) => Err(e),
    };

    if res.is_ok() {
        let response = format!("{}{}", status_line, res.unwrap());
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }


}

fn read_file(filename: &String) -> Result<String, io::Error> {
    let f = File::open(filename);
        
    let mut f = match f {

        Ok(file) => file,
        Err(error) => {
            return Err(error);
        },
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    Ok(contents)
}

