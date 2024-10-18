use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut hash = HashMap::new();
    let binding = String::from_iter(
        words
            .chars()
            .map(|c| {
                if !c.is_alphanumeric() && c != '\'' {
                    ' '
                } else {
                    c
                }
            })
            .collect::<Vec<char>>(),
    );
    let strip = binding.split(" ").collect::<Vec<&str>>();

    strip.iter().for_each(|word| {
        let cleaned_word = word.trim_matches('\'').to_lowercase();

        if !cleaned_word.is_empty() {
            *hash.entry(cleaned_word).or_insert(0) += 1;
        }
    });

    return hash;
}
