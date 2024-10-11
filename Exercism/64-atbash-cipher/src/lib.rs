/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let dataset = plain
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<char>>();

    let atbash_mapping = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .rev()
        .collect::<Vec<char>>();

    dataset
        .into_iter()
        .map(|c| {
            if c.is_alphabetic() {
                let index = (b'a'..=b'z').position(|b| b == c as u8).unwrap();
                atbash_mapping[index]
            } else {
                c
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>()) // collect each chunk into a string
        .collect::<Vec<String>>() // collect all chunks into a vector
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let dataset = cipher
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<char>>();

    let atbash_mapping = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();

    String::from_iter(
        dataset
            .into_iter()
            .map(|c| {
                if c.is_alphabetic() {
                    let index = 25 - (b'a'..=b'z').position(|b| b == c as u8).unwrap();
                    atbash_mapping[index]
                } else {
                    c
                }
            })
            .collect::<Vec<char>>(),
    )
}
