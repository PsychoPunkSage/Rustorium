struct occurence {
    character: char,
    count: usize,
}

pub fn encode(source: &str) -> String {
    let arr = source.chars().collect::<Vec<char>>();
    let mut vecc = Vec::new();
    let mut start = ' ';
    for i in arr {
        if start != i {
            vecc.push(occurence {
                character: i,
                count: 1,
            });
            start = i;
        } else {
            vecc.last_mut().unwrap().count += 1;
        }
    }

    vecc.iter()
        .map(|occ| {
            if occ.count > 1 {
                format!("{}{}", occ.count, occ.character)
            } else {
                format!("{}", occ.character)
            }
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    let arr = source
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                format!("-{} ", c)
            } else {
                format!("{}", c)
            }
        })
        .collect::<String>();
    let vecc: Vec<&str> = arr.split_whitespace().collect();

    vecc.into_iter()
        .map(|ele| {
            let split = ele.split("-").collect::<Vec<&str>>();
            let mut count = 1;
            if split[0] != "" {
                count = split[0].parse::<i32>().unwrap();
            }
            let mut strr = String::new();
            for _ in 0..count {
                strr = format!("{}{}", strr, split[1]);
            }
            strr
        })
        .collect::<String>()
}
