pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return vec![];
    }

    if digits.len() == len {
        return vec![digits.to_string()];
    } else {
        let digit_arr = digits.chars().into_iter().collect::<Vec<char>>();
        (0..digits.len() - len + 1)
            .into_iter()
            .map(|index| {
                let mut stri = String::new();
                for i in 0..len {
                    stri.push_str(&digit_arr[index + i].to_string());
                }
                stri
            })
            .collect::<Vec<String>>()
    }
}
