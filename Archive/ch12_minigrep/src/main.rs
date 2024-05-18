use std::env;
use std::process;

use ch12_minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let config = match Config::new(env::args()) {
        Ok(config) => config,
        Err(error) => {
            // eprintln:> Print errors in std error streams
            eprintln!("Problem parsing arguments: {}", error);
            println!("Usage::> Cargo r [QUERY] [FILENAME]");
            process::exit(1);
        }
    };

    /*
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
     */

    // We Only Bother about Error case.
    if let Err(e) = ch12_minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
