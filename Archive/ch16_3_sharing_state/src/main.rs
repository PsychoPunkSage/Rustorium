// use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(10);

    {
        // Need to Acquire `Lock` before accessing data
        let mut num = match m.lock() {
            Ok(n) => n,
            Err(poisoned) => poisoned.into_inner(),
        };
        println!("{:?}", num);
        println!("{:?}", m);
        *num = 19;
        println!("{:?}", num);
        println!("{:?}", m);
    }

    /*
    Internal Mutability
    - RefCell allows to mutate value inside `RC` smart pointer.
    - Mutex allows to mutate value inside `Arc` smart pointer. || MUTEX: uses INTERNAL mutability.
       |_> Can cause Deadlocks.
     */
    let counter = Arc::new(Mutex::new(0)); // Rc::> not thread safe.
    let mut handles = vec![];

    for _ in 0..10 {
        let count = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = count.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {:?}", counter.lock().unwrap());
}
