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
            *x1.lock() += 1;
        }
    });

    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            *x2.lock() -= 1;
        }
    });
}
