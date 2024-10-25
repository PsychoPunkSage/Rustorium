/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism {
    // This field is just to suppress compiler complaints;
    // feel free to delete it at any point.
    key: Vec<u8>,
}

impl Xorcism {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &Key) -> Xorcism {
        // AsRef::> This trait will allow the new function to accept various types (like &[u8], Vec<u8>, and even String) that can be cheaply converted to byte slices.
        Xorcism {
            key: key.as_ref().to_vec(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        // Perform XOR operation in-place.

        for (byte, key_byte) in data.iter_mut().zip(self.key.iter()) {
            *byte ^= key_byte;
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8> + '_
    where
        Data: Into<Vec<u8>>,
    {
        let d = data.into(); // Convert data into Vec<u8>

        d.into_iter()
            .zip(self.key.iter().cycle())
            .map(|(byte, key_byte)| byte ^ key_byte)
    }
}
