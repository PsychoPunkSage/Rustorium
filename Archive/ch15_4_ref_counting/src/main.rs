use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // For multiple Ownership :: only for SINGLE Treaded Program
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // .clone ::> Increases the reference count.
    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Count after creating c = {}", Rc::strong_count(&a));
}
