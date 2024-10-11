pub fn get_diamond(c: char) -> Vec<String> {
    let index = (c.to_ascii_lowercase() as u8 - b'a') as usize;

    let top_half: Vec<String> = (0..=index)
        .map(|i| {
            let chr = (b'a' + i as u8) as char;
            if i == 0 {
                format!(
                    "{:^width$}",
                    chr.to_ascii_uppercase(),
                    width = index * 2 + 1
                )
            } else {
                format!(
                    "{:^width$}",
                    format!(
                        "{}{}",
                        chr.to_ascii_uppercase(),
                        " ".repeat(2 * i - 1) + &chr.to_ascii_uppercase().to_string()
                    ),
                    width = index * 2 + 1
                )
            }
        })
        .collect();

    let bottom_half = top_half.iter().rev().skip(1).cloned().collect::<Vec<_>>();
    [top_half, bottom_half].concat()
}
