use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.chars().any(|c| !c.is_alphabetic() || c.is_uppercase()) || key.len() == 0 {
        return None;
    }

    let key_chars = key.chars().cycle(); // cycling the key is imp
    Some(String::from_iter(
        s.chars()
            .zip(key_chars)
            .map(|(s, k)| {
                if s.is_alphabetic() {
                    let ascii_index = s as u8 - b'a';
                    let increament = k as u8 - b'a';
                    let shift = (ascii_index + increament) % 26 + b'a';
                    shift as char
                } else {
                    s
                }
            })
            .collect::<Vec<_>>(),
    ))
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.chars().any(|c| !c.is_alphabetic() || c.is_uppercase()) || key.len() == 0 {
        return None;
    }

    let key_chars = key.chars().cycle(); // cycling the key is imp
    Some(String::from_iter(
        s.chars()
            .zip(key_chars)
            .map(|(s, k)| {
                if s.is_alphabetic() {
                    let ascii_index = s as u8 - b'a';
                    let increament = k as u8 - b'a';
                    let shift = (ascii_index + 26 - increament) % 26 + b'a';
                    shift as char
                } else {
                    s
                }
            })
            .collect::<Vec<char>>(),
    ))
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = String::from_iter(
        (0..100)
            .map(|_| (b'a' + rand::thread_rng().gen_range(0..26) as u8) as char)
            .collect::<Vec<char>>(),
    );

    (key.clone(), encode(&key, s).unwrap_or_else(|| "".into()))
}
