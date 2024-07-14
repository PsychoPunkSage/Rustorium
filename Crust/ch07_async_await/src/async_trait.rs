#[allow(dead_code, unused_variables)]
use std::future::Future;

fn main() {}

struct Request;
struct Response;

trait Service {
    fn call(&mut self, _: Request) -> impl Future<Output = Response>;
}

struct X;
impl Service for X {
    async fn call(&mut self, _: Request) -> Response {
        Response
    }
}

struct Y;
impl Service for Y {
    async fn call(&mut self, _: Request) -> Response {
        let z = [0; 1024];
        tokio::time::sleep(100).await;
        drop(z);
        Response
    }
}

struct FooCall<F>(F);

fn foo<S: Service>(x: &mut dyn Service) -> FooCall<typeof S::call> {
    let fut = x.call(Request);

    // Size of `Fut` depends on what stacked vatiable are there in the async block that was used for the future.
    // use `#[async_trait]` at places where trait impl an async fn
    /*
        This will re-write everything in the code...
            - Ex: impl Future<Output = Response> ===> Pin<Box<dyn Future<Output = Response>>>
            - Now here `Pin` is a pointer to Heap allocation of `Future`... and its size if known at compile time.
    */
}
