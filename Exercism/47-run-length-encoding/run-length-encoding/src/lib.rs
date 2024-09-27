struct Occurence {
    character: char,
    count: usize,
}

pub fn encode(source: &str) -> String {
    let arr = source.chars().collect::<Vec<char>>();
    let mut vecc = Vec::new();
    let mut start = '~';
    for i in arr {
        if start != i {
            vecc.push(Occurence {
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
    println!("Source::> {:?}", source);
    for c in source.chars() {
        println!("Char: '{}' is_numeric: {}", c, c.is_numeric());
    }

    if source.chars().any(|c| c.is_digit(10)) {
        let arr = source
            .chars()
            .map(|c| {
                if c.is_alphabetic() || c.is_whitespace() {
                    format!("-{}=", c)
                } else {
                    format!("{}", c)
                }
            })
            .collect::<String>();

        let mut vecc: Vec<&str> = arr.split("=").collect();
        println!("{:?}", vecc);
        vecc.pop();
        return vecc
            .into_iter()
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
            .collect::<String>();
    }
    source.to_string()
}
