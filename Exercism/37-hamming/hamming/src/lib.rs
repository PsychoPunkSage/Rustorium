/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let mut size = 0;
    let v1 = s1.chars().collect::<Vec<char>>();
    let v2 = s2.chars().collect::<Vec<char>>();

    for i in 0..s1.len() {
        if v1[i] != v2[i] {
            size += 1;
        }
    }

    Some(size)
}
