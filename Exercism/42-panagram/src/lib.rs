use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut hashset = HashSet::new();
    sentence
        .to_ascii_lowercase()
        .chars()
        .filter(|&c| c.is_alphabetic())
        .for_each(|ch| {
            hashset.insert(ch);
        });

    hashset.len() == 26
}
