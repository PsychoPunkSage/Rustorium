use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()

    // if input.len() <= 1 {
    //     return input.to_string();
    // }

    // let mut rev = String::with_capacity(input.len());
    // for chr in input.to_string().graphemes(false).rev() {
    //     rev.push_str(chr)
    // }
    // return rev;
}
