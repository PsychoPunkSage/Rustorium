use error_chain::error_chain;
use reqwest::blocking::Client;
use std::{
    fs::File,
    io::{Read, Write},
};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn create_json_file(path: &str, content: String) -> Result<()> {
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <FileLocation>", args[0]);
    }

    let client = Client::new();

    let user = String::from("PsychoPunkSage");
    let passwd: Option<String> = None;

    let mut response = client
        .get("http://httpbin.org/get")
        .basic_auth(user, passwd)
        .send()?;
    println!("Response: {:?}", response);

    let mut body = String::new();
    // read_to_string: reads the entire response (HTTP request) body and attempts to convert it into a String object.
    response.read_to_string(&mut body)?;

    match create_json_file(args[1].as_str(), body) {
        Ok(_) => println!("File Created"),
        Err(e) => println!("Error {:?}", e),
    }
    
    Ok(())
}
