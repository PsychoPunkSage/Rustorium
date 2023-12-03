use std::{fs::File, io::Write};
use error_chain::error_chain;

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

#[tokio::main]
async fn submain() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <FileLocation>", args[0]);
        return Ok(());
    }

    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {:?}", res.status());
    println!("Header: \n{:?}", res.headers());

    let body = res.text().await?;
    println!("Body: {:?}", body);

    match create_json_file("test.json", body) {
        Ok(_) => print!("File Created"),
        Err(e) => println!("Error: {:?}", e),
    };
    Ok(())
}

fn main() -> Result<()> {
    submain()?;
    Ok(())
}
