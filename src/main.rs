extern crate mime_guess;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;
use std::time::Instant;

#[derive(Serialize, Deserialize)]
struct Config {
    wwwroot: String,
    address: String,
}

fn main() {
    // Load Configuration Options.
    let o = match load_configuration() {
        Ok(v) => v,
        Err(_err) => {
            print!(
                "Failed to load configuration file. \r\n\
                 Please create a valid nano.json \
                 configuration file in the binary directory.");
            return;
        }
    };

    println!("\nNano server started!");
    println!("\nBinding address... {}", o.address);

    let listener = match TcpListener::bind(o.address) {
        Ok(socket) => socket,
        Err(error) => {
            println!("\nNano server was unable to start {}", error);
            return;
        }
    };

    println!("\nListening for connections...\n");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let now = Instant::now();
                handle_connection(stream, &o.wwwroot);
                println!("handeled connection in {}ms", now.elapsed().as_millis());
            }
            Err(e) => println!("Connection failed... {}", e),
        }
    }
}

fn load_configuration() -> Result<Config, io::Error> {
    let path = "nano.json".to_string();
    let config = read_file(&path)?;

    let v: Config = serde_json::from_str(&config)?;

    Ok(v)
}

fn handle_connection(mut stream: TcpStream, wwwroot: &String) {

    

    // Allocate 4kB buffer
    let mut buffer = [0; 1024];

    // Fill buffer from stream.
    stream.read(&mut buffer).unwrap();

    // Read utf8 buffer to String.
    let strget = String::from_utf8_lossy(&buffer);

    let chunks: Vec<_> = strget.split_whitespace().collect();

    if chunks.len() < 2 {
        not_found(&mut stream)
            .expect("not found error.");
        return;
    }

    let getfile = chunks[1]
        .replace("/", "\\")
        .trim_start_matches("\\")
        .to_string();

    let mainpage = "index.html".to_string();

    let ext = get_extension_from_filename(&getfile);
    let ext = match ext {
        Some(val) => val,
        None => ".html",
    };

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename, mime) = if buffer.starts_with(get) {
        (
            "HTTP/1.1 200 OK\r\n",
            &mainpage,
            mime_guess::get_mime_type_str(&mainpage),
        )
    } else {
        (
            "HTTP/1.1 200 OK\r\n",
            &getfile,
            mime_guess::get_mime_type_str(ext),
        )
    };

    let mime = match mime {
        Some(val) => val,
        None => "text/html;",
    };

    let mut full_path = String::with_capacity(128);
    full_path.push_str(&wwwroot);
    full_path.push_str(&filename);
    let contents = read_file(&full_path);

    let res = match contents {
        Ok(c) => c,
        Err(_err) => {
            not_found(&mut stream)
                .expect("not found error.");
            return;
        },
    };

    let response = format!(
        "{}Content-Type: {}\r\n\r\n{}",
        status_line,
        mime,
        res
    );

    res_ok(&mut stream, &response)
        .expect("res ok error.");

}

fn res_ok(stream: &mut TcpStream, response: &String) -> Result<(), io::Error>  {
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn not_found(stream: &mut TcpStream) -> Result<(), io::Error> {
    let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn read_file(filename: &String) -> Result<String, io::Error> {
    let f = File::open(filename);

    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            return Err(error);
        }
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
