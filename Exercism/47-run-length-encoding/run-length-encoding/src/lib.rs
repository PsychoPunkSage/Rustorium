// struct Occurence {
//     character: char,
//     count: usize,
// }

pub fn encode(source: &str) -> String {
    // let arr = source.chars().collect::<Vec<char>>();
    // let mut vecc = Vec::new();
    // let mut start = '~';
    // for i in arr {
    //     if start != i {
    //         vecc.push(Occurence {
    //             character: i,
    //             count: 1,
    //         });
    //         start = i;
    //     } else {
    //         vecc.last_mut().unwrap().count += 1;
    //     }
    // }

    // vecc.iter()
    //     .map(|occ| {
    //         if occ.count > 1 {
    //             format!("{}{}", occ.count, occ.character)
    //         } else {
    //             format!("{}", occ.character)
    //         }
    //     })
    //     .collect()

    source
        .chars()
        .fold(
            (Vec::new(), None),
            |(mut vecc, last_char): (Vec<(char, usize)>, Option<char>), c| {
                match last_char {
                    Some(lc) if lc == c => {
                        vecc.last_mut().unwrap().1 += 1; // Access and modify the count of the last element
                    }
                    _ => vecc.push((c, 1)), // Push a new character with count 1
                }
                (vecc, Some(c)) // Update the last character tracked
            },
        )
        .0 // Extract the vector
        .into_iter()
        .map(|(c, count)| {
            if count > 1 {
                format!("{}{}", count, c)
            } else {
                c.to_string()
            }
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    // if source.chars().any(|c| c.is_digit(10)) {
    //     let arr = source
    //         .chars()
    //         .map(|c| {
    //             if c.is_alphabetic() || c.is_whitespace() {
    //                 format!("-{}=", c)
    //             } else {
    //                 format!("{}", c)
    //             }
    //         })
    //         .collect::<String>();

    //     let mut vecc: Vec<&str> = arr.split("=").collect();
    //     vecc.pop();
    //     return vecc
    //         .into_iter()
    //         .map(|ele| {
    //             let split = ele.split("-").collect::<Vec<&str>>();
    //             let mut count = 1;
    //             if split[0] != "" {
    //                 count = split[0].parse::<i32>().unwrap();
    //             }
    //             let mut strr = String::new();
    //             for _ in 0..count {
    //                 strr = format!("{}{}", strr, split[1]);
    //             }
    //             strr
    //         })
    //         .collect::<String>();
    // }
    // source.to_string()

    let mut result = String::new();
    let mut count_str = String::new();

    for c in source.chars() {
        if c.is_digit(10) {
            count_str.push(c);
        } else {
            let count = count_str.parse::<usize>().unwrap_or(1);
            result.push_str(&c.to_string().repeat(count));
            count_str.clear();
        }
    }
    result
}
