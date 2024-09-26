pub fn translate(inputs: &str) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u']; // Only standard vowels here, not 'y'
    let words = inputs.split_whitespace();
    let mut result = Vec::with_capacity(inputs.len());

    for word in words {
        let mut chars = word.chars().collect::<Vec<_>>();

        // Rule 1: Words that start with a vowel or specific consonant combinations
        if VOWELS.contains(&chars[0])
            || (chars[0] == 'x' && chars[1] == 'r')
            || (chars[0] == 'y' && chars[1] == 't')
        // Handle 'yt' as a special case
        {
            result.push(format!("{}ay", word));
            continue;
        }

        // Rule 2: Handle words with "qu"
        if word.contains("qu") {
            let qu_index = word.find("qu").unwrap() + 2;
            result.push(format!("{}{}ay", &word[qu_index..], &word[..qu_index]));
            continue;
        }

        // Rule 3: General consonant cluster handling (treat 'y' as consonant when at the start)
        let vowel_pos = chars
            .iter()
            .position(|&c| VOWELS.contains(&c) || c == 'y') // 'y' acts as a vowel if it's not the first character
            .unwrap_or(chars.len());

        if vowel_pos > 0 {
            // Move the consonant cluster before the first vowel (or 'y') to the end
            let (consonants, rest) = chars.split_at(vowel_pos);
            let mut pig_latin_word: Vec<char> = rest.to_vec();
            pig_latin_word.extend_from_slice(consonants);
            pig_latin_word.push('a');
            pig_latin_word.push('y');
            result.push(pig_latin_word.into_iter().collect::<String>());
        } else {
            // For words with no vowels at all (edge case), just append "ay"
            chars.push('a');
            chars.push('y');
            result.push(chars.into_iter().collect::<String>());
        }
    }

    result.join(" ")
}
