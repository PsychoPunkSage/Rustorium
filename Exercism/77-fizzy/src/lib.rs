// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    logic: Box<dyn Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S: ToString>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
    {
        Matcher {
            logic: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        // todo!() doesn't actually work, here; () is not an Iterator
        // that said, this is probably not the actual implementation you desire
        iter.map(move |ele| {
            let mut result = String::new();
            for matcher in &self.matchers {
                if (matcher.logic)(ele) {
                    result.push_str(&matcher.subs);
                }
            }
            if result.is_empty() {
                result.push_str(&ele.to_string());
            }
            result
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Rem<Output = T> + From<u8> + PartialEq + Copy + ToString,
{
    Fizzy::new()
        .add_matcher(Matcher::new(
            |n: T| n % T::from(3_u8) == T::from(0_u8),
            "fizz",
        ))
        .add_matcher(Matcher::new(
            |n: T| n % T::from(5_u8) == T::from(0_u8),
            "buzz",
        ))
}
