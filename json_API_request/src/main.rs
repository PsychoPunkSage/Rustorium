use std::fmt;
use std::{fs::File, io::{Write, BufWriter}};
use error_chain::error_chain;
use serde::Deserialize; 
use reqwest::header::USER_AGENT; // Contains a string that allows identifying the requesting client’s software.
                                 // standard header used in HTTP requests to identify the user agent (e.g., browser, application) making the request.


error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
    }
}

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use `write!` to format the user information
        write!(f, "User {{ login: {}, id: {}}}", self.login, self.id)
    }
}

// We have content in form of <<Vector>>
fn create_json_file(path: &str, content: Vec<User>) -> Result<()> {

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for value in content.iter() {
        write!(&mut writer, "{}\n", value)?;
    }
    Ok(())
}

#[tokio::main]
async fn sub_main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <FIleLocation>", args[0]);
        return Ok(());
    }

    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers", /*owner="PsychoPunkSage" , repo="FOUNDRY_StableCoins"*/ owner="rust-lang-nursery" , repo="rust-cookbook");
    println!("{}", request_url);

    // Create a Client that will Make Request
    let client = reqwest::Client::new();

    // Creating Response
    let response = client.get(&request_url).header(USER_AGENT, "PsychoPunkSage").send().await?;

    let users: Vec<User> = response.json().await?;
    println!("Users: {:?}", users);

    match create_json_file(args[1].as_str(), users) {
        Ok(_) => println!("============\nFile Created\n============"),
        Err(e) => println!("Error: {:?}", e),
    };
    Ok(())
}

fn main() -> Result<()> {
    sub_main()?;
    Ok(())
}
