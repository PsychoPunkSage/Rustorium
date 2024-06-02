// #![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

pub struct StrSplit {}

impl StrSplit {
    pub fn new(haystack: &str, needle: &str) -> Self {}
}

// let x: StrSplit;
// for part in x {......}
impl Iterator for StrSplit {
    type Item = &str;

    fn next(&mut self) -> Option<Self::Item> {}
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
