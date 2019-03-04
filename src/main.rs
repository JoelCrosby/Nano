extern crate mime_guess;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod lib;

use std::io;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Instant;

#[derive(Serialize, Deserialize)]
struct Config {
    wwwroot: String,
    address: String,
}

fn main() {
    // Load Configuration Options.
    let o = match load_configuration("nano.json") {
        Ok(v) => v,
        Err(_err) => {
            print!(
                "Failed to load configuration file. \r\n\
                 Please create a valid nano.json \
                 configuration file in the binary directory."
            );
            return;
        }
    };

    println!("\nNano server started!");
    println!("Binding address... {}", o.address);
    println!("Serving dir {}", o.wwwroot);

    let listener = match TcpListener::bind(o.address) {
        Ok(socket) => socket,
        Err(error) => {
            println!("\nNano server was unable to start {}", error);
            return;
        }
    };

    println!("Listening for connections...\n");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let now = Instant::now();
                handle_connection(stream, &o.wwwroot);
                println!(" --  response ⚡ {}ms", now.elapsed().as_millis());
            }
            Err(e) => println!("Connection failed... {}", e),
        }
    }
}

fn load_configuration(path: &str) -> Result<Config, io::Error> {
    let config = lib::file::read_file(&path)?;
    let v: Config = serde_json::from_str(&config)?;

    Ok(v)
}

fn handle_connection(mut stream: TcpStream, wwwroot: &str) {

    let request = match lib::req::read_stream_to_request(&mut stream) {
        Ok(c) => c,
        Err(err) => {
            let msg = format!("An error occured while trying to read stream. \r\n{}", err);
            lib::res::res_internal_server_error(&mut stream, &msg).expect("unable to responde with 500");
            return;
        }
    };

    println!("req -> {}", request.path);

    let mainpage = "index.html".to_string();

    let ext = lib::file::get_extension_from_filename(&request.path);
    let ext = match ext {
        Some(val) => val,
        None => ".html",
    };

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename, mime) = if request.buffer.starts_with(get) {
        (
            "HTTP/1.1 200 OK\r\n",
            &mainpage,
            mime_guess::get_mime_type_str(&mainpage),
        )
    } else {
        (
            "HTTP/1.1 200 OK\r\n",
            &request.path,
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
    let contents = lib::file::read_file(&full_path);

    let res = match contents {
        Ok(c) => c,
        Err(_err) => {
            lib::res::res_not_found(&mut stream).expect("not found error.");
            return;
        }
    };

    let response = format!("{}Content-Type: {}\r\n\r\n{}", status_line, mime, res);

    println!("payload len {}", response.len());

    lib::res::res_ok(&mut stream, &response).expect("res ok error.");
}
