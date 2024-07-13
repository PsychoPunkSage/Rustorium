use std::sync::{Arc, Mutex};

use tokio::{self, sync::futures};

fn faulty_main() {
    let runtime = tokio::runtime::Runtime::new();
    runtime.block_om(async {
        // This signify that there is ONLY 1 FUTURE Possible.
        // Even though we await multiple time in this FuTURE... only CURRENT (i.e. 1) thread will be awaited... Other threads can't do anything.
        println!("Yo Server");

        let mut accept = tokio::net::TcpListener::bind("0.0.0.0:8080");
        let mut connections = futures::future::FuturesUnordered::new();
        loop {
            select! {
                stream <- (&mut accept).await => {
                    connections.push(handle_connection(stream));
                }
                _ <- (&mut connections).await => {}
            }
        }
    })
}

fn correct_main() {
    let runtime = tokio::runtime::Runtime::new();
    runtime.block_on(async {
        println!("Hi Server");

        let mut accept = tokio::net::TcpListener::bind("0.0.0.0:8080");
        while let Ok(stream) = accept.await {
            tokio::spawn(handle_connection(stream));
            // tokio::spwan - kind-of HOOK handle_connection() into the EXECUTOR.
            // so, now executor (i.e. tokio::runtime) now have 2 Future. i.e. 2 threads can run them at same time.
            // NOTE:: the future we spawn must be `Send` otherwise it can't be passed into different threads.
            //                                    `Static` cause `handle_connection` may have more `spawn statements` and there is chance that future `handle_connection` is over but that Spawn is still running
        }
    })
}

async fn handle_connection(_: tokio::net::TcpListener) {
    let x = Arc::new(Mutex::new(vec![1, 2, 3]));

    let x1 = Arc::clone(&x);
    let join_handle = tokio::spawn(async move {
        let _ = x1.lock();
        // blablablabla
        0
    });

    assert_eq!(join_handle.await.unwrap(), 0); // until and unless we await, we are not going to ge any value back....

    let x2 = Arc::clone(&x);
    let join_handle1 = tokio::spawn(async move {
        let _ = x2.lock();
        // blablablabla
        1
    });
    assert_eq!(join_handle1.await.unwrap(), 1);
}
