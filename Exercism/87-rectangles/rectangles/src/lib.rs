pub fn count(lines: &[&str]) -> u32 {
    if lines.len() < 2 {
        return 0;
    }
    let mut count = 0;

    for i in 0..lines.len() {
        let index_i = occurence(lines[i], '+');
        println!("-----------\n{:?}", index_i);
        for j in i + 1..lines.len() {
            let index_j = occurence(lines[j], '+');
            let c = collisions(&index_i, &index_j);
            println!("{:?}:{:?} => {}", index_i, index_j, c);
            if c == 2 {
                count += 1;
            }
            if c > 2 {
                count += c;
            }
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
