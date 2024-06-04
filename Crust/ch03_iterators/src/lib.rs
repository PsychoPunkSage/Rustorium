pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}
impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
    }
}
impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.inner = None;
            }
            let next_inner_item = self.outer.next()?.into_iter(); // Get inner items
            self.inner = Some(next_inner_item);
        }

        /*
        OR
        self.outer.next().and_then(|inner| inner.into_iter().next())
        */
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: Iterator + DoubleEndedIterator,
    O::Item: IntoIterator, // need to turn the inner items into Iterator as they are not originally an iterator.
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator, // IntoIter::Associated Type; It is the `Iterator` type of that inner iterator/item.
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next_back() {
                    return Some(i);
                }
                self.inner = None;
            }
            let next_inner_item = self.outer.next_back()?.into_iter(); // Get inner items
            self.inner = Some(next_inner_item);
        }
    }
}

pub fn flatten<O>(iter: O) -> Flatten<O::IntoIter>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["pps"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["pps", "ap"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["pps"], vec!["ap"]]).count(), 2);
    }
}
