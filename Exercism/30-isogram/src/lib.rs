use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let actual_len = candidate
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .count();

    let set_len = candidate
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<HashSet<char>>()
        .len();

    actual_len == set_len
}
