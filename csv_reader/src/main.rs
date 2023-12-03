use std::error::Error;
use csv;


fn main() {
    // println!("Hello, world!");
    if let Err(e) = read_csv_file("./customers.csv") {
        eprintln!("ERROR: {}", e)
    }
}

// Error handling: use 'Result' {Success: Ok(()), Error: Err(Box<dyn Error>)}
fn read_csv_file(csv: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(csv)?; // ?: alternative format for Match
    println!("Reader: {:?}", reader);

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    } 
    Ok(()) // -> his signals successful completion and tells the caller that everything went well.
}
 