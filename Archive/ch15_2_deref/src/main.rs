use std::ops::Deref;

// `Cus_Box` ::> Tuple Struct...
struct Cus_Box<T>(T);

impl<T> Cus_Box<T> {
    fn new(x: T) -> Cus_Box<T> {
        Cus_Box(x)
    }
}

impl<T> Deref for Cus_Box<T> {
    type Target = T;

    // fn deref(&self) -> &T {
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x); // Stores Pointer in Stack
    let w = Cus_Box::new(x);
    println!("{}", y);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_ne!(5, y);
    assert_eq!(5, *z);
    assert_eq!(5, *w); // *w = *(w.deref())

    // Deref Coersion
    let m = Cus_Box::new(String::from("AP"));
    hello(&m);
    // &Cus_Box<String> :: auto implements `deref` => &String => &str
    hello(&(*m)[..]);

    /*
    DEREF is performed:
    - &T -> &U
    - &mut T -> &mut U
    - &mut T -> &U
    but not for: =&T -> &mut U || Borrow rules
     */
}

fn hello(name: &str) {
    println!("Hello, {}!!", name);
}
