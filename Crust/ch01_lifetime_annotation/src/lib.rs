// #![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
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
            remainder: Some(haystack),
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
        if let Some(ref mut remainder) = self.remainder {
            // let remainder = self.remainder.as_mut()?;    -->    .as)mut() :: returns Option<&mut T>
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delim = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delim)
            } else {
                self.remainder.take()
                // .take() ::> impl on Option<T>
                // if Option == None ::> returns None
                // if Option == Some ::> returns Some + Set the value to none
            }
        } else {
            None
        }
    }
}

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &c.to_string())
        .next()
        .expect("StrSplitter always returns at-least one result")
}

#[cfg(test)]
mod tests {
    use crate::StrSplit;

    #[test]
    fn it_works() {
        let haystack = "a b c d e f";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        // println!("letters: {:?}", letters);
        assert_eq!(letters, vec!["a", "b", "c", "d", "e", "f"]);
        let letters = StrSplit::new(haystack, " ");
        // println!("letters: {:?}", letters);
        // println!(
        //     "letters: {:?}",
        //     vec!["a", "b", "c", "d", "e", "f"].into_iter()
        // );
        assert!(letters.eq(vec!["a", "b", "c", "d", "e", "f"].into_iter()));
    }
}
