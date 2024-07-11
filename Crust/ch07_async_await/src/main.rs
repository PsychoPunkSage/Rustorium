#[allow(dead_code, unused_variables)]

fn main() {
    println!("Hello, world!");

    // type(x) != usize
    // .await will return something which will eventually convert to `usize`.
    let x = foo();
}

async fn foo() -> usize {
    0
}

/*

async fn foo() -> usize {}

fn foo() -> impl Future<Output = usize> {
    async {}
}

*/
