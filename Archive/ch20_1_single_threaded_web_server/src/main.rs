use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(e) => {
            println!("Connection refused");
            println!("Could not bind to port:\nError{}", e);
            return;
        }
    };

    for stream in listener.incoming() {
        // .incoming() ::> iterator over the connections being received on this listener
        let stream = match stream {
            Ok(st) => st,
            Err(e) => {
                println!("Connection refused");
                println!("Could not accept connection:\nError{}", e);
                return;
            }
        };

        // println!("Connection Established!!\n{:?}", steam);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Buffer to hold data that is being read.
    let mut buffer = [0; 1024];

    let _ = match stream.read(&mut buffer) {
        Ok(_) => {
            // Check if the request is for the root path
            let get = b"GET / HTTP/1.1\r\n";

            let (status_line, page_name) = if buffer.starts_with(get) {
                // Serve index.html
                ("HTTP/1.1 200 OK", "index.html")
            } else {
                // Serve 404.html
                ("HTTP/1.1 404 NOT FOUND", "404.html")
            };

            let content = match fs::read_to_string(page_name) {
                Ok(string) => string,
                Err(e) => {
                    println!("Could not read from file:\nError{}", e);
                    return;
                }
            };
            let resp = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                content.len(),
                content
            );
            stream.write(resp.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            println!("Could not read from stream:\nError{}", e);
            return;
        }
    };

    /*
    REQUEST Body:

    HTTP-Version Status-Code Reason-Phrase CRLF
    headers CRLF
    message-body

    ex: HTTP/1.1 200 OK\r\n\r\n
    */
}
