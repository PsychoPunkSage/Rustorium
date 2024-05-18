fn main() {
    // `Box` smart-pointer allocate memory in "HEAP"
    // a `pointer` to the data is stored in stack

    /*
    USAGE:
    - Don't know the size of data @compile time
    - Want to transfer the ownership of large amount of data.
     */
    let b = Box::new(12);
    println!("b::> {}", b);
}
