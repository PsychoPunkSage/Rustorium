use add_one;

fn main() {
    let num = 3;
    println!("Hello, world! {}, plus one {}", num, add_one::add_one(num));
}
