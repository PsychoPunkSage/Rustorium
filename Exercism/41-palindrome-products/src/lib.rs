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

use std::u64;

// use rayon::prelude::*;
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut max_p = None;
    let mut min_p = None;

    // (min..=max)
    //     // .into_par_iter() // Parallel iterator: as the entire thing is Parallel
    //     .for_each(|v| {
    //         (v..=max)
    //             // .into_par_iter() // Parallel iterator from the rayon crate
    //             .for_each(move |i| {
    //                 let product = i * v;
    //                 if is_palindrome(product) {
    //                     if product < min_p.unwrap_or(u64::MAX) {
    //                         min_p = Some(product);
    //                     }
    //                     if product > max_p.unwrap_or(u64::MIN) {
    //                         max_p = Some(product);
    //                     }
    //                 }
    //             })
    //     });

    // match (min_p, max_p) {
    //     (Some(min), Some(max)) => Some((Palindrome(min), Palindrome(max))),
    //     _ => None,
    // }

    for i in min..=max {
        if i % 10 == 0 {
            continue;
        };
        for j in min..=max {
            if j % 10 == 0 {
                continue;
            };
            let p = i * j;
            if is_palindrome(p) {
                if p < min_p.unwrap_or(u64::MAX) {
                    min_p = Some(p);
                }
                if p > max_p.unwrap_or(u64::MIN) {
                    max_p = Some(p);
                }
            }
        }
    }
    match (min_p, max_p) {
        (Some(min), Some(max)) => {
            Some((Palindrome::new(min).unwrap(), Palindrome::new(max).unwrap()))
        }
        _ => None,
    }
}

fn is_palindrome(v: u64) -> bool {
    let s = v.to_string();
    s == s.chars().rev().collect::<String>()
}
