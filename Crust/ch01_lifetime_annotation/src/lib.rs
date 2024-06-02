// #![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

#[derive(Debug, PartialEq)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    // We can use `StrSplit` for as long as `input string` are valid !!!
    /*
    Me to Complier...
        - I will give you `StrSplit` with lifetime 'a as long as you ensure `haystack` and `delimiter` is around for 'a lifetime

    impl<'a> ::> this says that this impl is generic over this lifetime 'a...
    */
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

/*
// Compiler know this is wrong but it doesn't know the right answer
fn multiply(x: (), y: i32) -> i32 {
    x * y
}
*/

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.delimiter.is_empty() {
            None
        } else {
            let rest = self.remainder;
            // &'a str       &'static str
            // We can assign a lifetime with `same type` of lifetime or `longer` lifetime. VICE VERSA not true.
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
        let mut letters = StrSplit::new(haystack, " ");
        println!("{:?}", letters.next());
        assert!(letters.eq(vec!["a", "b", "c", "d", "e", "f"].into_iter()))
        // assert_eq!(2 + 2, 3);
    }
}
