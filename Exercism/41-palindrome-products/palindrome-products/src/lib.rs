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

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let palindromes = (min..=max).flat_map(|v| {
        (v..=max).filter_map(move |i| {
            if is_palindrome(i * v) {
                Some(i * v)
            } else {
                None
            }
        })
    });

    match (palindromes.clone().min(), palindromes.max()) {
        (Some(min_p), Some(max_p)) => Some((Palindrome(min_p), Palindrome(max_p))),
        _ => None,
    }
}

fn is_palindrome(v: u64) -> bool {
    let chars = v.to_string().chars().collect::<Vec<_>>();
    let test = (0..=chars.len() / 2)
        .map(|i| {
            if chars[i] == chars[chars.len() - 1 - i] {
                true
            } else {
                false
            }
        })
        .collect::<Vec<bool>>();

    if test.iter().any(|&a| a == false) {
        return false;
    }

    true
}
