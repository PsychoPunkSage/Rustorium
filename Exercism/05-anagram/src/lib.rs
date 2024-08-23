use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let l_word = word.len();
    let sword = get_sorted_strings(word);

    let mut hashset = HashSet::new();

    for &w in possible_anagrams.iter() {
        if word.to_lowercase() != w.to_lowercase() {
            let test_word = get_sorted_strings(w);

            if l_word == w.len() && test_word == sword {
                hashset.insert(w);
            }
        }
    }

    hashset
}

fn get_sorted_strings(word: &str) -> String {
    let mut chars = word
        .graphemes(true)
        .map(|g| g.to_lowercase().to_string())
        .collect::<Vec<String>>();
    chars.sort_by(|a, b| b.cmp(a));
    chars.into_iter().collect::<String>()
}
