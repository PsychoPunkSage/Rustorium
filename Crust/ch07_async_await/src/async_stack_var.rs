#[allow(dead_code, unused_variables)]
use std::future::Future;
use tokio;

#[tokio::main]
async fn main() {
    let mut x /*StateMachine; actually */ = foo();
    x.await; // actially ::> StateMachine::await(&mut x)

    /*
        - At this point `Future` contains lot of Data... and passing it around will cause a huge overhead..
        - Cause wherever the future is passed (ist can be selsct!{}, other function etc.)... we need to store the data repeatdly..
        - AVOID:: Do Box-ing, this will provide a `pointer to the Heap allocation` which makes things easier...

        - thats why `Spawn` is cool,,, as it stores the |>> pointer to the Future <<| and awaits on that pointer....
    */
}

////// async fn is a chunked computation.
async fn foo() {
    // fn foo() -> impl Future<Output = ()> /* StateMachine */ {
    // Chunk 1
    {
        let mut x = [0; 1024];
        let z = vec![];
        let fut = tokio::fs::read_into("file.dat", &mut x[..]);
    }
    // fut.await
    yield; // really: return
           // Here the `stack` frame goes away. i.e. we loose `x`... cause if the task is INCOMPLETE; await will `yield`.

    // chunk 2
    {
        // by this time `await` is finished.
        let n = fut.output(); // self.fut -> Reorganised...
        println!("{}", x[..n]); // self.x -> Reorganised... << self === StateMachine >>
    }
}

/*
When we write an `async fn`... The compiler generated sort-of `enum` of the `StateMachine`.

enum StateMachine {
    Chunk1 {
        x : [u8; 1024],
        fut: tokio::fs::ReadIntoFuture<'x> // self-referential.
    },

    Chunk2 {
        x : [u8; 1024], // NOT a COPY.... think it as UNION for convolution....
    },
}

* each Chunk has State (i.e.) local var OR Values that needs to be kept across await points.
*/
