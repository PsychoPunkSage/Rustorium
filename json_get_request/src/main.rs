use error_chain::error_chain;
use std::io::prelude::*;
use std::fs::File;

// Define Custom Error
error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn create_json_file(path: &str, content: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}


fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <fileLocation>", args[0]);
    };

    /* blocking: For applications wishing to only make a few HTTP requests */
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    println!("Response: \n{:?}", res);
    
    let mut body = String::new();
    // read_to_string: reads the entire response (HTTP request) body and attempts to convert it into a String object.
    res.read_to_string(&mut body)?;

    match create_json_file(args[1].as_str(), &body) {
        Ok(_) => println!("Successfully created"),
        Err(e) => eprintln!("Error: {}", e),
    };

    println!("status: {}", res.status());
    println!("header: {:?}", res.headers());
    println!("Body: \n{}", body);
    Ok(())
}
