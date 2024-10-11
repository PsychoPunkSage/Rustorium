pub fn get_diamond(c: char) -> Vec<String> {
    let index = (b'a'..=b'z')
        .position(|b| b == c.to_ascii_lowercase() as u8)
        .unwrap();

    let mut result = vec![];

    for i in (0..=index).rev() {
        println!("entered here");
        let mut string = String::new();
        let c = ((i as u8 + b'a') as char).to_ascii_uppercase();

        if i != 0 {
            string = format!(
                "{}{}{}{}{}",
                " ".repeat(index - i),
                c,
                " ".repeat(2 * i - 1),
                c,
                " ".repeat(index - i)
            );

            if i == index {
                result.push(string);
            } else {
                result.insert(0, string.clone());
                result.push(string);
            }
        } else {
            string = format!("{}{}{}", " ".repeat(index - i), c, " ".repeat(index - i));
            if index != 0 {
                result.insert(0, string.clone());
                result.push(string);
            } else {
                result.push(string);
            }
        }
    }
    result
}
