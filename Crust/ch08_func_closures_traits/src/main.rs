fn main() {
    println!("Hello, world!");
    let x = bar; // function item
    let x1 = bar1::<i32>; // generic function item
                          // x = bar1::<u32>(); cannot reassign a with different generic.
    let x2 = bar2; // function item

    println!(
        "SIZE: x - {} || x1 - {}",
        std::mem::size_of_val(&x),
        std::mem::size_of_val(&x)
    );

    /*
    Funtion Item:
        - "Zero sized" value that is only carried around at compile time the reference a unique function.
        - Its just an `Identifier`.
        - it uniquely identifies a particular instance of a function.

    Function Pointer:
        - pointer to a function with a given signature.
    */
    bazz(bar1::<i32>);
    bazz(bar1::<u32>);

    // `Function Item` & `Function Pointer` are different from one another. BUT `Function Item(s)`  are coersible inside the `Function Pointer`.
    // Demonstrated above.
}

fn bar() {}
fn bar1<T>(_: u32) -> u32 {
    0
}
async fn bar2() {}

fn bazz(f: fn(u32) -> u32) {
    // Now here the `function item` is properly converted to `fn pointer`.
    println!("{}", std::mem::size_of_val(&f));
}
