extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File; // Open File
use std::io::copy;
use std::io::BufReader; // Efficient reading of file
use std::time::Instant;

fn main() {
    // The prog is supposed to Expect 3 Args
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`"); // error println!
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    // println!("Input data: ${:?}", input);
    let output = File::create(args().nth(2).unwrap()).unwrap();
    // println!("Output data: ${:?}", output);

    let mut encoder = GzEncoder::new(output, Compression::default());
    // println!("Encoding data: ${:?}", encoder);
    let start = Instant::now();
    
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    // println!("Output data: ${:?}", output);

    println!(
        "Source len: {:?}", input.get_ref().metadata().unwrap().len()
    );

    println!(
        "Target len: {:?}", output.metadata().unwrap().len()
    );

    println!("Elapsed time: {:?}", start.elapsed());
}