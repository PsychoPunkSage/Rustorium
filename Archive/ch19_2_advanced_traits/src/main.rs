use std::ops::Add;
// pub trait Add<Rhs = Self> {
//     type Output;
//     // Self and rhs must have the same type
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

// Associated type:: type which are unknown until we implement the trait.
pub trait Iterator {
    type Item;
    /*
    data: vec[integers] || Item = integer
    data: String        || Item = char
     */
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait IteratorGeneric<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = i32; // Since we declared `i32` we cannot implement any other type

    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
    }
}

impl IteratorGeneric<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
        Some(1)
    }
}

impl IteratorGeneric<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(899)
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// =================================================================================================
struct MM(u32);
struct M(u32);

impl Add<M> for MM {
    type Output = MM;
    fn add(self, rhs: M) -> MM {
        MM(self.0 + (rhs.0 * 1000))
    }
}

// =================================================================================================
trait Pilot {
    fn fly(&self);
    fn walk();
}
trait Wizard {
    fn fly(&self);
    fn walk();
}
struct Human;

impl Human {
    fn fly(&self) {
        println!("**Waving hands furiously**")
    }
    fn walk() {
        println!("I'm walking");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your Captain AP speaking")
    }
    fn walk() {
        println!("On track")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!!!")
    }
    fn walk() {
        println!("Buzz off")
    }
}

// =================================================================================================
use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// // =================================================================================================
// struct Wrapper(Vec<String>);

// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }
fn main() {
    assert_eq!(
        Point { x: 1, y: 8 } + Point { x: 2, y: 4 },
        Point { x: 3, y: 12 }
    );
    // --------------------------------------------------------------
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    Human::walk(); // Associated fn of HUMAN will be called
    <Human as Wizard>::walk();
    <Human as Pilot>::walk();

    // --------------------------------------------------------------
    let p = Point { x: 1, y: 2 };
    p.outline_print();

    // --------------------------------------------------------------
    // let w = Wrapper(vec!["Yo Boys!!".to_string(), " Ap is here :)".to_string()]);
    // println!("{}", w);
}
