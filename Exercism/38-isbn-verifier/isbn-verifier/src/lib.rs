/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let lenn = isbn
        .chars()
        .filter(|&c| c.is_alphanumeric())
        .collect::<Vec<char>>()
        .len();

    if lenn == 10 {
        let v: Vec<char> = isbn
            .chars()
            .filter(|&c| c.is_numeric() || c == 'X')
            .collect();
        if v.len() == 10 {
            let s = (0..=9)
                .map(|i| {
                    if v[i] == 'X' {
                        10 * (10 - i) as u32
                    } else {
                        v[i].to_digit(10).unwrap() * (10 - i) as u32
                    }
                })
                .collect::<Vec<u32>>()
                .into_iter()
                .sum::<u32>();
            return s % 11 == 0;
        }
        return false;
    }
    false
}
