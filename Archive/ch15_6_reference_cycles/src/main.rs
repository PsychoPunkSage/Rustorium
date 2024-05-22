use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

////===============================================================\\\\
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // Weak: Version of `Rc` which holds a non-owning reference.
    child: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("rc(a) initial= {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("rc(a) after b = {}", Rc::strong_count(&a));
    println!("rc(b) initial = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(item) = a.tail() {
        *item.borrow_mut() = Rc::clone(&b);
    }
    println!("rc(b) after a = {}", Rc::strong_count(&b));
    println!("rc(a) after a = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail()); // ERROR: Stack-overflow; infinite loop

    ////===============================================================\\\\
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),
    });

    println!("leaf parent: {:#?}", leaf.parent.borrow().upgrade()); // borrow: cause of RefCell || upgrade: dereference Weak pointer
    println!("leaf value: {:#?}", leaf.value);
    print!(
        "leaf<initial> strong: {}, weak: {}\n",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    println!("branch parent: {:#?}", branch.parent.borrow().upgrade());
    print!(
        "leaf<after branch> strong: {}, weak: {}\n",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    // println!("branch: {:#?}", branch);

    // Allocate parent to `leaf`
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // downgrade: create `Weak` Smart pointer.
    print!(
        "branch strong: {}, weak: {}\n",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
    print!(
        "leaf strong: {}, weak: {}\n",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!("leaf parent: {:#?}", leaf.parent.borrow().upgrade());
    print!(
        "leaf strong: {}, weak: {}\n",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
