use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    /*
    RefCell::>
    - Single ownership over the data it contains.
    - Enforces borrowing rules @runtime || `Box` : @compile-time
    - Single threaded program

    ___________________________________________________________________
    | Box     | Single Owner   | mutable/immutable borrow @compile-time
    | vs      |
    | Rc      | Multiple Owner | immutable borrow @compile-time
    | vs      |
    | RefCell | Single Owner   | mutable/immutable borrow @runtime
    ===================================================================

    - Mutating a value inside an immutable value is `interior mutability`
     */

    // ERRORS::>
    // let a = 10;
    // let b = &mut a; // cannot mutate immutable variable `a`

    // let mut c = 12;
    // let d = &c;
    // *d = 13; // cannot assign to `*d`, which is behind a `&` reference `d` is a `&` reference, so the data it refers to cannot be written

    // Rc with RefCell
    let value = Rc::new(RefCell::new(5));
    println!("{:?}", value);

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    println!("a:: {:#?}", a);

    *value.borrow_mut() += 10;

    println!("a:: {:#?}", a);
}
