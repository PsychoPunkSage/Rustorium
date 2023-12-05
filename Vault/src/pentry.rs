use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Write, BufReader};

#[derive(Serialize, Deserialize, Debug)]
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

    pub fn from_json_to_rust_ds(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string) /*.expect("Failed to Deserialize JSON")*/  // Deserializes: JSON string -> Rust DS
    }

    #[allow(dead_code)]
    pub fn from_user_input() -> Self {
        println!("Enter Service: ");
        let mut service = String::new();
        /* stdin(): standard input stream */
        io::stdin()
            .read_line(&mut service)
            .expect("Failed to read Service"); // read Input

        println!("Enter username: ");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read Username"); // read Input

        println!("Enter password: ");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read Password");

        ServiceInfo::new(
            service.trim().to_string(), // <<trim()>> method removes any leading and trailing whitespace from the string
            username.trim().to_string(),
            password.trim().to_string(),
        )
    }

    pub fn to_json(&self) -> String {
        // Serializes: Rust DS -> JSON string.
        serde_json::to_string(&self).expect("Failed to Serialize JSON")
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());

        // <<OpenOptions>> interface for configuring how a file should be opened.
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("password.json")
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) {
                    eprintln!("Error writing file: {}", e);
                } else {
                    println!("Successfully updated DB.");
                }
            }
            Err(e) => println!("Error opening file: \n{}", e),
        }
    }

}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}


pub fn read_pass_from_file() -> Result<Vec<ServiceInfo>, io::Error> {
    let file = File::open("passwords.json")?;
    let reader = BufReader::new(file);

    let mut services = Vec::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let Ok(service_info) = ServiceInfo::from_json_to_rust_ds(&json_string) {
                services.push(service_info);
            }
        }
    }

    Ok(services)
}
