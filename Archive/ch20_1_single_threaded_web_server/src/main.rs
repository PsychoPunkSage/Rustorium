use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(e) => panic!("Could not bind to port:\nError{}", e),
    };

    for stream in listener.incoming() {
        // .incoming() ::> iterator over the connections being received on this listener
        let stream = match stream {
            Ok(st) => st,
            Err(e) => {
                println!("Connection refused");
                panic!("Could not accept connection:\nError{}", e)
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

            if buffer.starts_with(get) {
                // Serve index.html
                let web_page = match fs::read_to_string("index.html") {
                    Ok(string) => string,
                    Err(e) => {
                        println!("Could not read from file:\nError{}", e);
                        return;
                    }
                };

                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    web_page.len(),
                    web_page
                );
                stream.write(resp.as_bytes()).unwrap();
                stream.flush().unwrap();
            } else {
                // Serve 404.html
                let error_page = match fs::read_to_string("404.html") {
                    Ok(string) => string,
                    Err(e) => {
                        println!("Could not read from file:\nError{}", e);
                        return;
                    }
                };

                let status_line = "HTTP/1.1 404 NOT FOUND";
                let resp = format!(
                    "{}\r\nContent-Length: {}\r\n\r\n{}",
                    status_line,
                    error_page.len(),
                    error_page
                );
                stream.write(resp.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
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
