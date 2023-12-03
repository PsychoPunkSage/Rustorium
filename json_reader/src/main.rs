use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>, // Multiple Structs of "Paragraphs"
}

fn read_json(raw_json: &str) -> Article {
    let parsed = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

// fn read_json_file(path: &str) -> Result<String, std::io::Error> {
//     let mut file = fs::File::open(path)?;
//     let mut contents = String::new();

//     file.read_to_string(&mut contents)?;
//     Ok(contents);
// }

fn main() {
    let json = r#"
    {
        "article": "Work with JSON on Rust",
        "author": "PsychoPunkSage",
        "paragraphs": [
            {
                "name": "A paragraph"
            },
            {
                "name": "Paragraph 1"
            },
            {
                "name": "Paragraph 2"
            },
            {
                "name": "Paragraph 3"
            }
        ]
    }"#;

    let parsed: Article = read_json(json);
    for line in 0..parsed.paragraphs.len() {
        println!(
            "\nPara:{} => Para is: {}",
            line, parsed.paragraphs[line].name
        );
    }
}
