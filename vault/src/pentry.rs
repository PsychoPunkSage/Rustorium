use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}
impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        // Create new ServiceInfo obj.
        ServiceInfo {
            service,
            username,
            password,
        }
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        // Deserializes: JSON string -> Rust DS
        serde_json::from_str(json_string) 
    }

    pub fn to_json(&self) -> String {
        // Serializes: Rust DS -> JSON string.
        serde_json::to_string(&self).expect("Failed to serialize to JSON")
    }

    #[allow(dead_code)]
    pub fn from_user_input() -> Self {
        println!("Enter Password Entry:");
        let mut service = String::new();
        /* stdin(): standard input stream */
        io::stdin()
            .read_line(&mut service)
            .expect("Failed to read line"); // read Input

        println!("Enter Username:");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line"); // read Input

        println!("Enter Password:");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line"); // read Input

        ServiceInfo::new(
            service.trim().to_string(), // <<trim()>> method removes any leading and trailing whitespace from the string
            username.trim().to_string(),
            password.trim().to_string(),
        )
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());

        // <<OpenOptions>> interface for configuring how a file should be opened.
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json")
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) {
                    eprintln!("Error writing to file: {}", e);
                } else {
                    println!("Successfully wrote to passwords.json");
                }
            }
            Err(e) => eprintln!("Error opening file: {}", e),
        }
    }
}

pub fn read_pass_from_file() -> Result<Vec<ServiceInfo>, io::Error> {
    //  1. Open the File
    let file = File::open("passwords.json")?;

    //  2. Read the file
    let reader = io::BufReader::new(file);

    // 3. Create the Service Vec
    let mut services = Vec::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let Ok(service_info) = ServiceInfo::from_json(&json_string) {
                services.push(service_info);
            }
        }
    }

    Ok(services)
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // orces any buffered output to be written to the console immediately. This ensures that the prompt is displayed before the program waits for input.

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // read/accept input

    input.trim().to_string()
}

