/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    // word.to_ascii_lowercase()
    //     .chars()
    //     .map(|c| match c {
    //         a if vec!['a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't'].contains(&a) => 1,
    //         a if vec!['d', 'g'].contains(&a) => 2,
    //         a if vec!['b', 'c', 'm', 'p'].contains(&a) => 3,
    //         a if vec!['f', 'h', 'v', 'w', 'y'].contains(&a) => 4,
    //         a if vec!['k'].contains(&a) => 5,
    //         a if vec!['j', 'x'].contains(&a) => 8,
    //         a if vec!['q', 'z'].contains(&a) => 10,
    //         _ => 0,
    //     })
    //     .collect::<Vec<u64>>()
    //     .into_iter()
    //     .sum()

    const SCORES: [u64; 26] = [
        1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
    ];
    word.to_lowercase()
        .chars()
        .filter(|x| 'a' <= *x && *x <= 'z')
        .map(|c| SCORES[(c as u8 - b'a') as usize])
        .sum()
}
