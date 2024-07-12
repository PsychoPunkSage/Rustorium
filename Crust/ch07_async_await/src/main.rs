use std::{fs::read_to_string, future::Future, io::BufRead};

#[allow(dead_code, unused_variables)]

/*
Future::>
    - descrive a series of steps that will happen at some point in the future.
    - It won't until it is awaited.
*/

fn main() {
    println!("Hello, world!");

    // If NOT using async/await
    let read_from_terminal = std::thread::spawn(move || {
        let mut x = std::io::Stdin::lock(std::io::stdin());
        for lines in x.lines() {
            // Do something
        }
    });

    let read_from_network = std::thread::spawn(move || {
        let mut x = std::net::TcpListener::bind("0.0.0.0:8000").unwrap();
        while let Ok(stream) = x.accept() {
            // Do something on stream
            // Create `handle_connection` for all the stream <So you don't have to wait for slow client and can serve FAST client>
        }
    });

    // USING Async
    let network = read_from_network();
    let terminal = read_from_terminal();

    select! {
        // This macro in present in various library.
        // WAITS on various FUTURES....
        // and tells which one got executed first.

        stream <- network.await => {
            // do something on stream
        }

        line <- terminal.await => {
            // do something with line
        }
    }

    // type(x) != usize
    // .await will return something which will eventually convert to `usize`.
    let x = foo2();
}

async fn foo1() -> usize {
    println!("Inside Foo");
    0
}

fn foo2() -> impl Future<Output = usize> {
    async {
        println!("Inside Foo2");
        read_to_string("file1").await;
        /*
        let fut = read_to_string("file1");
        let x = loop {
            if let Some(result) = fut.try_check_complete() {
                break result;
            } else {
                fut.try_make_progress();
                yield;
            }
        }
        */

        /*
            YIELD:
        Suppose :: main() wait for foo2(); foo2() waits for read_to_string(); read_to_string() waits for foo1()

        Whenever we yield; it returns to the TOP of callstack <main()>
        Whenever progress check <try_check_complete()> is performed; it starts from "bottom most" thing that previously called yield.
        */
        println!("Inside Foo2");
        read_to_string("file2").await;
        println!("Inside Foo2");
        read_to_string("file3").await;
        println!("Inside Foo2");
        read_to_string("file4").await;
        0
    }
}

/*
SYNTAX RELATION/SIMPLEFICATION

*   async fn foo() -> usize {}

*   fn foo() -> impl Future<Output = usize> {
        async {}
    }
*/
