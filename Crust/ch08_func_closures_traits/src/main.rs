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
    */
}

fn bar() {}
fn bar1<T>() {}
async fn bar2() {}
