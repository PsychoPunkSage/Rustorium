use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    for (key, value) in h {
        for char in value {
            let lower_case_char = char.to_ascii_lowercase();
            if let Some(count) = result.get_mut(&lower_case_char) {
                *count += key;
            } else {
                result.insert(lower_case_char, *key);
            }
        }
    }

    result
}

pub fn main() {
    let input = input_from(&[(1, vec!['A'])]);

    let expected = expected_from(&[('a', 1)]);

    assert_eq!(expected, transform(&input));
}

fn input_from(v: &[(i32, Vec<char>)]) -> BTreeMap<i32, Vec<char>> {
    v.iter().cloned().collect()
}

fn expected_from(v: &[(char, i32)]) -> BTreeMap<char, i32> {
    v.iter().cloned().collect()
}
