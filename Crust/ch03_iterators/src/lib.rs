// FLATTEN: it is compile time construct. We need to supply type @compile time.
pub trait ExtIterator: Iterator {
    fn pps_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator;
}

impl<T> ExtIterator for T
where
    T: Iterator,
{
    fn pps_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}
impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
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
            if let Some(inner_iter) = self.front_iter.as_mut() {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }
            if let Some(next_inner_item) = self.outer.next() {
                // Get inner items
                self.front_iter = Some(next_inner_item.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
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
            if let Some(ref mut inner_iter) = self.back_iter {
                // if let Some(inner_iter) = &mut self.back_iter {
                if let Some(i) = inner_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }
            if let Some(next_back_inner_item) = self.outer.next_back() {
                self.back_iter = Some(next_back_inner_item.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
            } // Get inner items
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

    #[test]
    fn reverse_two() {
        assert_eq!(
            flatten(std::iter::once(vec!["pps", "ap"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["ap", "pps"]
        );
    }

    #[test]
    fn reverse_two_wide() {
        assert_eq!(
            flatten(vec![vec!["pps"], vec!["ap"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["ap", "pps"]
        );
    }

    #[test]
    fn extensive_back_next() {
        let mut flatten = Flatten::new(vec![vec!["1", "2", "3"], vec!["4", "5", "6"]].into_iter());
        assert_eq!(flatten.next(), Some("1"));
        assert_eq!(flatten.next_back(), Some("6"));
        assert_eq!(flatten.next(), Some("2"));
        assert_eq!(flatten.next_back(), Some("5"));
        assert_eq!(flatten.next(), Some("3"));
        assert_eq!(flatten.next_back(), Some("4"));
        assert_eq!(flatten.next(), None);
        assert_eq!(flatten.next_back(), None);
    }

    #[test]
    fn inf() {
        let mut flatten = flatten((0..).map(|i| 0..i));
        /*
        OUTER   InnerRange  INNER
        0          0..0      [empty]
        1          0..1      [0]
        2          0..2      [0, 1]
        3          0..3      [0, 1, 2]
        .....
        */
        assert_eq!(flatten.next(), Some(0));
        assert_eq!(flatten.next(), Some(0));
        assert_eq!(flatten.next(), Some(1));
        assert_eq!(flatten.next(), Some(0));
        assert_eq!(flatten.next(), Some(1));
        assert_eq!(flatten.next(), Some(2));
    }

    #[test]
    fn flatten_flatten_deep() {
        assert_eq!(flatten(flatten(vec![vec![vec![1, 2, 3, 4]]])).count(), 4);
        assert_eq!(
            vec![vec![vec![1, 2, 3, 4]]].into_iter().flatten().count(),
            1
        );
        assert_eq!(
            vec![vec![vec![1, 2, 3, 4]]]
                .into_iter()
                .flatten()
                .flatten()
                .count(),
            4
        );
    }
}
