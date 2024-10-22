// Trait with an associated type
trait Converter {
    type Output;

    fn convert(&self) -> Self::Output;
}

// Implement the Converter trait for i32 -> String conversion
impl Converter for i32 {
    type Output = String;

    fn convert(&self) -> Self::Output {
        format!("Converted i32: {}", self)
    }
}

// Implement the Converter trait for bool -> i32 conversion
impl Converter for bool {
    type Output = i32;

    fn convert(&self) -> Self::Output {
        if *self {
            1
        } else {
            0
        }
    }
}

// Implement the Converter trait for &str -> usize (string length)
impl Converter for &str {
    type Output = usize;

    fn convert(&self) -> Self::Output {
        self.len()
    }
}

fn main() {
    let num = 42;
    let flag = true;
    let text = "Hello";

    // Calls convert() on different data types with associated outputs
    println!("{}", num.convert()); // i32 -> String
    println!("{}", flag.convert()); // bool -> i32
    println!("{}", text.convert()); // &str -> usize
    println!("{}", Converter::convert(&false));
    println!("{}", Converter::convert(&"false"));
}
