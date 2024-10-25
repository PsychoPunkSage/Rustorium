// pub fn count(lines: &[&str]) -> u32 {
//     if lines.len() < 2 {
//         return 0;
//     }
//     let mut count = 0;

//     for i in 0..lines.len() {
//         let index_i = occurence(lines[i], '+');
//         println!("-----------\n{:?}", index_i);
//         for j in i + 1..lines.len() {
//             let index_j = occurence(lines[j], '+');
//             let (mut c, vecc) = collisions(&index_i, &index_j);
//             if j - i != 1 {
//                 for k in i + 1..j {
//                     for l in 0..vecc.len() {
//                         let ch = lines[k].chars().collect::<Vec<_>>()[vecc[l]];
//                         if ch != '+' && ch != '|' {
//                             c -= 1;
//                         }
//                     }
//                 }
//             }
//             let win = vecc.windows(2);
//             for w in win {
//                 if w[1] - w[0] != 1 {
//                     if lines[i].chars().collect::<Vec<char>>()[w[0] + 1..w[1]]
//                         .iter()
//                         .any(|c| c != &'+' && c != &'-')
//                         || lines[j].chars().collect::<Vec<char>>()[w[0] + 1..w[1]]
//                             .iter()
//                             .any(|c| c != &'+' && c != &'-')
//                     {
//                         c -= 1;
//                     }
//                 }
//             }

//             println!("{:?}:{:?} => {}", index_i, index_j, c);
//             if c == 2 {
//                 count += 1;
//             }
//             if c > 2 {
//                 count += c;
//             }
//         }
//     }
//     count
// }

// fn occurence(s: &str, c: char) -> Vec<usize> {
//     s.char_indices()
//         .filter(|(_, ch)| *ch == c)
//         .map(|(i, _)| i)
//         .collect()
// }

// fn collisions(v1: &Vec<usize>, v2: &Vec<usize>) -> (u32, Vec<usize>) {
//     let mut count = 0;
//     let mut collision = Vec::new();
//     for i in v1 {
//         for j in v2 {
//             if i == j {
//                 count += 1;
//                 collision.push(*i);
//             }
//         }
//     }

//     (count, collision)
// }

pub fn count(lines: &[&str]) -> u32 {
    if lines.len() < 2 {
        return 0;
    }

    let mut rectangle_count = 0;
    let plus_positions: Vec<Vec<usize>> = lines.iter().map(|line| occurences(line, '+')).collect();

    for row1 in 0..lines.len() {
        for start_col in 0..plus_positions[row1].len() {
            for end_col in start_col + 1..plus_positions[row1].len() {
                let col1 = plus_positions[row1][start_col];
                let col2 = plus_positions[row1][end_col];

                for row2 in row1 + 1..lines.len() {
                    if plus_positions[row2].contains(&col1) && plus_positions[row2].contains(&col2)
                    {
                        if is_complete_rectangle(lines, row1, row2, col1, col2) {
                            rectangle_count += 1;
                        }
                    }
                }
            }
        }
    }

    rectangle_count
}

fn occurences(line: &str, target: char) -> Vec<usize> {
    line.char_indices()
        .filter_map(|(i, ch)| if ch == target { Some(i) } else { None })
        .collect()
}

fn is_complete_rectangle(
    lines: &[&str],
    row1: usize,
    row2: usize,
    col1: usize,
    col2: usize,
) -> bool {
    is_valid_edge(&lines[row1][col1..=col2])
        && is_valid_edge(&lines[row2][col1..=col2])
        && is_valid_side(lines, row1, row2, col1, '|')
        && is_valid_side(lines, row1, row2, col2, '|')
}

fn is_valid_side(
    lines: &[&str],
    start_row: usize,
    end_row: usize,
    col: usize,
    target: char,
) -> bool {
    (start_row + 1..end_row).all(|row| {
        lines[row]
            .chars()
            .nth(col)
            .map_or(false, |ch| ch == target || ch == '+')
    })
}

fn is_valid_edge(segment: &str) -> bool {
    segment.chars().all(|ch| ch == '-' || ch == '+')
}
