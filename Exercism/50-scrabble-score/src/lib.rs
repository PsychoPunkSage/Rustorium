/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_ascii_lowercase()
        .chars()
        .map(|c| match c {
            a if vec!['a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't'].contains(&a) => 1,
            a if vec!['d', 'g'].contains(&a) => 2,
            a if vec!['b', 'c', 'm', 'p'].contains(&a) => 3,
            a if vec!['f', 'h', 'v', 'w', 'y'].contains(&a) => 4,
            a if vec!['k'].contains(&a) => 5,
            a if vec!['j', 'x'].contains(&a) => 8,
            a if vec!['q', 'z'].contains(&a) => 10,
            _ => 0,
        })
        .collect::<Vec<u64>>()
        .into_iter()
        .sum()
}
