use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.filename);
    let contents = fs::read_to_string(config.filename)?;

    println!("\nMatching Contents:");
    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for lines in results {
        println!("{}", lines);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // }
        // // index = 0 ==> Binary Path
        // let query = args[1].clone();
        // let filename = args[2].clone();

        args.next(); // Path to our program
        let query = match args.next() {
            Some(arg) => arg, // owned string
            None => return Err("Didn't find query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't find filename string"),
        };

        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Duct name.
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
        Rust:
safe, fast, productive.
Trust me.
        ";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
