pub struct Flatten<O> {
    outer: O,
}
impl<O> Flatten<O> {
    fn new(iter: O) -> Self {
        Flatten { outer: iter }
    }
}
impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.outer.next().and_then(|inner| inner.into_iter().next())
    }
}

pub fn flatten<O>(iter: O) -> Flatten<O> {
    Flatten::new(iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["pps"])).count(), 1);
    }
}
