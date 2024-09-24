/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            return Some(Palindrome(value));
        }
        None
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

use rayon::prelude::*;
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let palindromes: Vec<u64> = (min..=max)
        .into_par_iter() // Parallel iterator: as the entire thing is Parallel
        .flat_map(|v| {
            (v..=max)
                .into_par_iter() // Parallel iterator from the rayon crate
                .filter_map(move |i| {
                    let product = i * v;
                    if is_palindrome(product) {
                        Some(product)
                    } else {
                        None
                    }
                })
        })
        .collect();

    let min_palindrome = palindromes.iter().min().copied();
    let max_palindrome = palindromes.iter().max().copied();

    match (min_palindrome, max_palindrome) {
        (Some(min), Some(max)) => Some((Palindrome(min), Palindrome(max))),
        _ => None,
    }
}

fn is_palindrome(v: u64) -> bool {
    let s = v.to_string();
    s == s.chars().rev().collect::<String>()
}
