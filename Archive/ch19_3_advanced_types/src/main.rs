struct Age(u32);
struct ID(u32);

fn main() {
    type Kilometers = i32;

    let x: i32 = 109;
    let y: Kilometers = 109;

    println!("x + y = {}", x + y);

    // Type Alias
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_thunk(f: Thunk) {
        // --snip --
    }

    fn returns_thunk() -> Thunk {
        // --snip --
    }

    // NEVER type
    // fn never_return() -> ! {}
    // Ex
    let game = true;
    let mut guess = String::new();
    while game {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // `continue` never_return.
        };
    }

    let none: Option<i32> = None;
    let a = match none {
        Some(val) => val,
        None => panic!("Panic `NEVER` return anything"),
    };

    loop {
        println!("Loop NEVER return anything UNLESS Break statement is introduced");
    }
}

// Generic TRAITS ALWAYS implement `Sized` feature... which ensures that memory size of object is known at compile time.
fn generic<T: ?Sized>(t: &T) {
    // ? -> may/may not be `Sized` type
    // Since T can potentially be UN-Sized So, the argument type is put behind REFERENCE
}
