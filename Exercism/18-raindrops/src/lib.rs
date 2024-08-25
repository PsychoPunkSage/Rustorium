pub fn raindrops(n: u32) -> String {
    //////// NOOB

    // let mut result = String::new();

    // if n % 3 == 0 {
    //     result.push_str("Pling");
    // }
    // if n % 5 == 0 {
    //     result.push_str("Plang");
    // }
    // if n % 7 == 0 {
    //     result.push_str("Plong");
    // }

    // if result.is_empty() {
    //     result.push_str(&n.to_string());
    // }

    // result

    //////// Bit OPTIMIZED

    // let string = vec![3, 5, 7]
    //     .iter()
    //     .map(|&i| {
    //         if n % i == 0 {
    //             match i {
    //                 3 => format!("Pling"),
    //                 5 => format!("Plang"),
    //                 7 => format!("Plong"),
    //                 _ => format!(""),
    //             }
    //         } else {
    //             format!("")
    //         }
    //     })
    //     .collect::<String>();

    // if string.is_empty() {
    //     return n.to_string();
    // } else {
    //     return string;
    // }

    let res = [3, 5, 7]
        .iter()
        .filter_map(|&i| match i {
            3 if n % i == 0 => Some("Pling"),
            5 if n % i == 0 => Some("Plang"),
            7 if n % i == 0 => Some("Plong"),
            _ => None,
        })
        .collect::<String>();

    res.is_empty().then(|| n.to_string()).unwrap_or(res)
}
