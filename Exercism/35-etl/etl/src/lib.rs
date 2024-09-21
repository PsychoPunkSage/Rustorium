use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    h.iter().for_each(|(a, b)| {
        b.iter().for_each(|c| {
            result.insert(c.to_ascii_lowercase(), *a);
        })
    });

    result
}
