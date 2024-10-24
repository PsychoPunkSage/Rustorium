pub fn count(lines: &[&str]) -> u32 {
    if lines.len() < 2 {
        return 0;
    }
    let mut count = 0;

    for i in 0..lines.len() {
        let mut plus_index: Vec<Vec<usize>> = Vec::new();
        plus_index.push(occurence(lines[i], '+'));
        for j in i..lines.len() {
            plus_index.push(occurence(lines[j], '+'));
            let c = collisions(&plus_index[0], &plus_index[1]);
            if c > 1 {
                count += c - 1;
                break;
            }
            plus_index.pop();
        }
    }

    count
}

fn occurence(s: &str, c: char) -> Vec<usize> {
    s.char_indices()
        .filter(|(_, ch)| *ch == c)
        .map(|(i, _)| i)
        .collect()
}

fn collisions(v1: &Vec<usize>, v2: &Vec<usize>) -> u32 {
    let mut count = 0;
    for i in v1 {
        for j in v2 {
            if i == j {
                count += 1;
            }
        }
    }
    count
}
