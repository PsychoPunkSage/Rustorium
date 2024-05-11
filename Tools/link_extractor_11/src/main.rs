use std::fs::File;
use std::io::Write;

use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn create_file_logs(path: &str, content: &String) -> Result<()> {
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn submain() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <File location>", args[0]);
    }

    let res = reqwest::get("https://www.youtube.com/watch?v=i7ABlHngi1Q")
        .await?
        .text()
        .await?; // .text() :: Response -> String

    match create_file_logs(args[1].as_str(), &res) {
        Ok(_) => println!("Logs Created"),
        Err(e) => eprintln!("Error: {}", e),
    }; 

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
    Ok(())
}

fn main() -> Result<()> {
    submain()?;
    Ok(())
}
