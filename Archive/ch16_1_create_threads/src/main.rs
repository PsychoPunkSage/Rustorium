use std::{thread, time::Duration};

fn main() {
    /*
    - Programs are executed as processes for running and OS are configured to run multiple processes at the same time.
    - Multiple parts of our program can run independently, so, we need to configure them inside `threads`. => increases complexity.... as we can't maintain order of calls
            * Race conditions, where threads are accessing data or resources in an inconsistent order
            * Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
            * Bugs that happen only in certain situations and are hard to reproduce and fix reliably


    */

    //// Execution order is not deterministic... so OUTPUT may look different in different systems.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("No. {}; spawned thread !!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // Spawned thread will be printed 1st and then main thread.

    for i in 1..5 {
        println!("No. {}; main thread !!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // blocks a currently running thread and ask the thread associated with `handle` to end....

    ////========================================================\\\\
    let v = vec![1, 2, 3, 4, 5, 6];
    // need to take ownership of `v` cause RUST don't know how long the thread will run' So there is a high change that `v` might get invalid while running the thread.
    let handle1 = thread::spawn(move || {
        // Force thread to take ownership of `v`
        println!("vector: {:?}", v);
    });
    handle1.join().unwrap();
}
