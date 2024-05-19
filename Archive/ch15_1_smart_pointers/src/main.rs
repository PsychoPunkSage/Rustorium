use List::{Cons, Nil};

// Cons List
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // `Box` smart-pointer allocate memory in "HEAP"
    // a `pointer` to the data is stored in "stack"

    /*
    USAGE:
    - Don't know the size of data @compile time
    - Want to transfer the ownership of large amount of data.
     */
    let b = Box::new(12);
    println!("b::> {}", b);

    // Cons List
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list::> {:#?}", list);
}
