use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn read_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut string = String::new();

    // match f.read_to_string(&mut string) {
    //     // Read the file and store the contents inside the `string`
    //     Ok(_) => Ok(string),
    //     Err(e) => Err(e),
    // }

    f.read_to_string(&mut string)?;
    Ok(string)
}

fn read_file_shorthand() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    // `?` can only be used in those fn which return a `Result` or `Options`.
    Ok(s)
}

fn main() {
    /////////////////
    // Backtracing //
    /////////////////
    a();

    // Generic Error handling.
    let f = File::open("hello.txt");
    // let f = File::open("hello.txt").unwrap(); // return Ok() value | else it will `panic`
    // let f = File::open("hello.txt").expect("Failed to open file"); // Fail with a custom error message.

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // graceful error handling
            ErrorKind::NotFound => match File::create("./hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening file: {:?}", other_error),
        },
    };

    println!("{:?}", f); // File { fd: 3, path: "absolute/path/to/hello.txt", read: false, write: true }
}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't Pass 22.")
    }
}
