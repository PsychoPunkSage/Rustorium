fn main() {
    let mut x = 5;
    println!("x: {}", x);
    x = 10;
    println!("x: {}", x);
    let y = 5;
    let y = 10; // Shadowing
    println!("y: {}", y);

    const A: u32 = 1_000;
    /*
    - Can't be the result of a value that could only be computed at runtime.
    - Can't use `mut` keyword here
    */

    let length = "abhdnfl pqpe";
    println!("length of 'length': {}", length.len());

    // ||||||||||
    // Data Types
    // ||||||||||

    let a = 98_222;
    let b = 0xff; // Hex
    let c = 0o77; // Octadecimal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Bytes -> u8 ONLY
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);

    // |||||||||||||||
    // Compound Types
    // |||||||||||||||

    let tup = ("Yo", 90);
    let (stri, num) = tup; // Destructutring
    let num = tup.1; // index starts from `0`

    let errors = [400, 404, 500];
    let not_found = errors[1];
    let bytes = [0; 8]; // Create an array of len=8 with all values=0

    // if-else inside `let`
    let condition = true;
    let number = if condition { 3 } else { 90 };

    // loop inside `let`

    let mut counter = 0;
    let a = loop {
        counter += 1;
        if counter == 11 {
            break counter * 2;
        }
    };

    //  Types of loop::> while || For
    let mut cont = 3;
    while cont != 0 {
        cont -= 1;
        println!("{}", cont);
    }

    let a = [10, 20, 30, 40, 50];
    for ele in a.iter() {
        println!("Val {}", ele);
    }
}
