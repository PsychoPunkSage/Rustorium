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
            let (mut c, vecc) = collisions(&index_i, &index_j);

            if j - i != 1 {
                for k in i + 1..j {
                    for l in 0..vecc.len() {
                        let ch = lines[k].chars().collect::<Vec<_>>()[vecc[l]];
                        if ch != '+' && ch != '|' {
                            c -= 1;
                        }
                    }
                }
            }

            let win = vecc.windows(2);
            for w in win {
                if w[1] - w[0] != 1 {
                    if lines[i].chars().collect::<Vec<char>>()[w[0] + 1..w[1]]
                        .iter()
                        .any(|c| c != &'+' && c != &'-')
                        || lines[i].chars().collect::<Vec<char>>()[w[0] + 1..w[1]]
                            .iter()
                            .any(|c| c != &'+' && c != &'-')
                    {
                        c -= 1;
                    }
                }
            }

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

fn collisions(v1: &Vec<usize>, v2: &Vec<usize>) -> (u32, Vec<usize>) {
    let mut count = 0;
    let mut collision = Vec::new();
    for i in v1 {
        for j in v2 {
            if i == j {
                count += 1;
                collision.push(*i);
            }
        }
    }
    (count, collision)
}
