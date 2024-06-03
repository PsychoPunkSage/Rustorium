// #![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

/*
MULTIPLE LIFETIMES::
- Used when need to store multiple reference and they are not the same.
- when you want to return one without tying it to other.
*/

#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    // We can use `StrSplit` for as long as `input string` are valid !!!
    /*
    Me to Complier...
        - I will give you `StrSplit` with lifetime 'haystack as long as you ensure `haystack` and `delimiter` is around for 'haystack lifetime

    impl<'haystack> ::> this says that this impl is generic over this lifetime 'haystack...
    */
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
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

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;

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

/*
String -> &str [cheap : AsRef]
&str -> String [expensive: Clone/mem-copy]
*/

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &c.to_string())
        .next()
        .expect("StrSplitter always returns at-least one result")
}

#[cfg(test)]
mod tests {
    use crate::{until_char, StrSplit};

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

    #[test]
    fn until_char_works() {
        assert_eq!(until_char("hello World", 'o'), "hell");
    }
}