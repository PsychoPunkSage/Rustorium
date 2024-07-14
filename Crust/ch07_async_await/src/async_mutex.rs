#[allow(dead_code, unused_variables)]
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio;
use tokio::sync::Mutex;

async fn main() {
    let x = Arc::new(Mutex::new(0));

    let x1 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            // *x1.lock() += 1;
            let x = x1.lock().unwrap();
            tokio::fs::read_to_string("file").await;
            *x += 1;
        }
    });

    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            *x2.lock().unwrap() -= 1;
        }
    });
}

/*
Tokio::Mutex v/s std::Mutex

1. i will lock X1
2. then I will await `read_to_string`... i.e. it will yield!!

3. Now It will go to run other Future i.e. other spawn...
4. it tries to grab `x2 lock`.... but the lock is already held by `X1`
5. So, the std lib Lock will just BLOCK THE THREAD, as it don't know what to do next....
    - This also means that executor's one htread is blocked... it doesn't get to complete `read_to_string`
    - this means the lock is never dropped and hence we get a DEADLOCK...


FOR Tokio::Mutex OR async aware Lock
1. if it fails to take `X2 lock` it will `yield`.... this means execution goes back to `read_to_string`... once the await is complete, X1 will be released....
*/
