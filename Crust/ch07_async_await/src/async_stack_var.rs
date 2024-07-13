#[allow(dead_code, unused_variables)]
use std::future::Future;
use std::num::ParseIntError;
use tokio;

fn main() {}

async fn foo() {
    // Chunk 1
    {
        let mut x = [0; 1024];
        let fut = tokio::fs::read_into("file.dat", &mut x);
    }
    // fut.await
    yield; // really: return

    // chunk 2
    {
        let n = fut.output();
        println!("{}", x[..n]);
    }
}
