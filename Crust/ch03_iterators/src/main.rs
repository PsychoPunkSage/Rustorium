trait Iterator {
    // We used `Associated type` here cause,
    // We expect the a particular `type` to have ONLY a single implementation.
    // Associated types reduce the amount of extra generic type parameters you will be using.
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

trait Service<Request> {
    // Have different impl for a particular `type` of parameter.
    fn doer(&mut self, request: Request);
}

fn main() {
    let mut iter = vec![1, 2, 3, 4].into_iter();
    while let Some(e) = iter.next() {
        println!("{}", e);
    }
}
