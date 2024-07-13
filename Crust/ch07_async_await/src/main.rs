mod server;

use std::{future::Future, io::BufRead};
use tokio;

#[allow(dead_code, unused_variables)]
/*
Future::>
    - descrive a series of steps that will happen at some point in the future.
    - It won't until it is awaited.
*/
#[tokio::main]
async fn main() {
    /*
    Below Runtime is derived version of
    #[tokio::main]
    async fn main() {....}
     */
    // let runtime = tokio::runtime::Runtime::new();
    // runtime.block_on(async {
    println!("Hello, world!");

    // If NOT using async/await
    let read_from_terminal = std::thread::spawn(move || {
        let mut x = std::io::Stdin::lock(&std::io::stdin());
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
    let mut network = read_from_networks();
    let mut terminal = read_from_terminals();
    let mut foo = foo2(_);

    let mut f1 = tokio::fs::File::open("foo"); // std library `fs` don't support async functions.
    let mut f2 = tokio::fs::File::create("bar");
    let copy = tokio::io::copy(&mut f1, &mut f2);

    // CONCAT all the awaits.
    let files: Vec<_> = (0..100)
        .map(|x| tokio::fs::read_to_string(format!("file{}", x)))
        .collect(); // Not moving anywhere untill the all the Futures are returned.
    let (file1, file2, file3) = tokio::join!(files[0], files[1], files[2]); // Give OS the task to deal with reading/writing operation in most efficient way.
                                                                            /*
                                                                            let file1 = files[0].await;
                                                                            let file2 = files[1].await;
                                                                            let file3 = files[2].await;

                                                                               - Here instead of allowing OS to deal with things in most efficient way, we are making things sequesntically unproductive.
                                                                            */
    let file_bytes = futures::future::try_join_all(files);

    loop {
        select! {
            // This macro in present in various library.
            // WAITS on various FUTURES....
            // and tells which one got executed first.
            // i.e. it tries all the branches and tell us which one got executed.

            stream <- (&mut network).await => {
                // do something on stream
            }

            line <- (&mut terminal).await => {
                // do something with line
            }

            foo <- (&mut foo).await => {
                // 1. Wait on `file1` reading :: yield happens
                // 2. control moves to the Top of call stack (i.e. select{})
                // 3. then select will again check for any progress in `stream` or `line`
                //      - if any of them changes; then run the required codes.
                //      - if nothing changes then continue Awaiting `Foo2``
                // 4. after this control shifts back to the bottom of the call stack.
            }

            _ <- copy.await {
                // Do something
                // Suppose 1 GB file has to be copied...
                // There is high chance that other branches get executed and `file is partially copied`.
                // MUST BE AWARE ABOUT THIS error CASE.
            }
        }
    }
    // });

    // type(x) != usize
    // .await will return something which will eventually convert to `usize`.
    // let x = foo2();
}

async fn foo1() -> usize {
    println!("Inside Foo");
    0
}

async fn read_to_strings(_: &str) {}
async fn read_from_networks() {}
async fn read_from_terminals() {}

fn foo2(cancel: tokio::sync::mpsc::Receiver<()>) -> impl Future<Output = usize> {
    async {
        println!("Inside Foo2");
        read_to_strings("file1").await;
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
        read_to_strings("file2").await;
        println!("Inside Foo2");
        // To Drop await.
        race! {
            done <- read_to_strings("file3").await {
                // continue your job....
            }
            cancel <- cancel.await {
                return 0;
            }
        }
        println!("Inside Foo2");
        read_to_strings("file4").await;
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

/*
ASYNC Overhead:
    - There is no significant overhead for making a fn `async`.
    - async IO Read VS IO Read
        *
*/
