/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.chars().any(|c| c.is_alphabetic() && c != 'X') {
        return false;
    }

    let filtered: Vec<char> = isbn
        .chars()
        .filter(|&c| c.is_numeric() || c == 'X')
        .collect();

    if filtered.len() != 10 {
        return false;
    }

    if filtered[..9].contains(&'X') {
        return false;
    }

    let sum: u32 = filtered
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            if c == 'X' {
                10 * (10 - i) as u32
            } else {
                c.to_digit(10).unwrap() * (10 - i) as u32
            }
        })
        .sum();

    sum % 11 == 0
}
