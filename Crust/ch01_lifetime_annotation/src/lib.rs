// #![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

pub struct StrSplit {
    remainder: &str,
    delimiter: &str,
}

impl StrSplit {
    pub fn new(haystack: &str, delimiter: &str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

// let x: StrSplit;
// for part in x {......}
impl Iterator for StrSplit {
    type Item = &str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.delimiter.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StrSplit;

    #[test]
    fn it_works() {
        let haystack = "a b c d e f";
        let letters = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e", "f"].into_iter())
    }
}
