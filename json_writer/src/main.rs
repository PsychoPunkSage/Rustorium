use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn create_json_file(path: &str, content: String) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage {} <filename>", args[0])
    }

    let article = Article {
        article: String::from("Struct to JSON write"),
        author: String::from("PsychoPunkSage"),
        paragraphs: vec![
            Paragraph{
                name: String::from("Para 1"),
            }, 
            Paragraph{
                name: String::from("Para 2"),
            },
            Paragraph{
                name: String::from("Para 2"),
            }
        ],
    };

    // ----Main---- \\
    let json = serde_json::to_string(&article).unwrap();
    println!("_______\n|JSON:|\n-------\n {}", json);

    match create_json_file(args[1].as_str(), json) {
        Ok(_) => println!("Successfully created"),
        Err(error) => println!("Error: {}", error),
    };
}
