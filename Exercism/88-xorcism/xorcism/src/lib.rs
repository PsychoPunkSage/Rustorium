use std::{borrow::Borrow, iter::Cycle, slice::Iter};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    // This field is just to suppress compiler complaints;
    // feel free to delete it at any point.
    key: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        // AsRef::> This trait will allow the new function to accept various types (like &[u8], Vec<u8>, and even String) that can be cheaply converted to byte slices.
        Xorcism {
            key: key.as_ref().into_iter().cycle(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        // Perform XOR operation in-place.

        data.into_iter()
            .for_each(|ele| *ele ^= self.key.next().unwrap());
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + Captures<'a> + 'b
    where
        Data: IntoIterator,
        Data::IntoIter: 'b,
        Data::Item: Borrow<u8>,
    {
        data.into_iter()
            .map(move |b| b.borrow() ^ self.key.next().unwrap())
    }
}
// Workaround for E0700 due to `impl Trait` not being able to have multiple lifetimes
// See: https://stackoverflow.com/a/50548538
pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}
