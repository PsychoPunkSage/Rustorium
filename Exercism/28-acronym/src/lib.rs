pub fn abbreviate(phrase: &str) -> String {
    let binding = phrase
        .trim()
        .chars()
        .into_iter()
        .map(|mut c| {
            if !c.is_alphabetic() && c != '\'' {
                c = ' ';
            }
            c
        })
        .collect::<String>();
    let binding = binding
        .split_whitespace()
        .filter(|&s| !s.is_empty())
        .collect::<Vec<&str>>();

    binding
        .iter()
        .map(|&word| {
            *word
                .chars()
                .into_iter()
                .collect::<Vec<char>>()
                .first()
                .unwrap()
        })
        .collect::<Vec<char>>()
        .into_iter()
        .map(|letter| letter.to_string().to_uppercase())
        .collect::<String>()
}
