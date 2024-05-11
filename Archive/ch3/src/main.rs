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
}
