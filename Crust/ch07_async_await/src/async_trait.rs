#[allow(dead_code, unused_variables)]
use std::future::Future;

fn main() {}

struct Request;
struct Response;

trait Service {
    fn call(_: Request) -> impl Future<Output = Response>;
}
