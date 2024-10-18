pub fn rotate(input: &str, key: u8) -> String {
    String::from_iter(
        input
            .chars()
            .map(|c| {
                if c.is_alphabetic() {
                    let ascii_offset = if c.is_lowercase() {
                        'a' as u8
                    } else {
                        'A' as u8
                    };
                    let rotated_ascii = (c as u8 - ascii_offset + key) % 26 + ascii_offset;
                    rotated_ascii as char
                } else {
                    c
                }
            })
            .collect::<Vec<_>>(),
    )
}
