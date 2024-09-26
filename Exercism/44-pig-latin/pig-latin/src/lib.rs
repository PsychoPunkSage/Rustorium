pub fn translate(inputs: &str) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u']; // Only standard vowels here, not 'y'
    let words = inputs.split_whitespace();
    let mut result = Vec::with_capacity(inputs.len());

    for word in words {
        let mut chars = word.chars().collect::<Vec<_>>();

        // Rule 1
        if VOWELS.contains(&chars[0])
            || (chars[0] == 'x' && chars[1] == 'r')
            || (chars[0] == 'y' && chars[1] == 't')
        {
            result.push(format!("{}ay", word));
            continue;
        }

        // Rule 2
        if word.contains("qu") {
            let qu_index = word.find("qu").unwrap() + 2;
            result.push(format!("{}{}ay", &word[qu_index..], &word[..qu_index]));
            continue;
        }

        // Rule 3
        let vowel_pos = chars
            .iter()
            .position(|&c| VOWELS.contains(&c) || (c == 'y' && chars[0] != 'y'))
            .unwrap_or(chars.len());

        println!("Hey ...{}", vowel_pos);
        if vowel_pos > 0 {
            println!("hey");
            let (consonants, rest) = chars.split_at(vowel_pos);
            let mut pig_latin_word: Vec<char> = rest.to_vec();
            pig_latin_word.extend_from_slice(consonants);
            pig_latin_word.push('a');
            pig_latin_word.push('y');
            result.push(pig_latin_word.into_iter().collect::<String>());
        } else {
            println!("hey22");
            chars.push('a');
            chars.push('y');
            result.push(chars.into_iter().collect::<String>());
        }
    }

    result.join(" ")
}
