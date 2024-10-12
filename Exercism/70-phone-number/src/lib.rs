pub fn number(user_number: &str) -> Option<String> {
    let num = user_number
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<Vec<char>>();

    if (num.len() == 11 && num[0] == '1') || (num.len() == 10 && num[0] != '1') {
        if num.len() == 11 {
            let num: Vec<char> = num[1..].iter().map(|c| *c).collect();
            if check_n(num[0]) && check_n(num[3]) {
                return Some(String::from_iter(num));
            }
            return None;
        }

        if check_n(num[0]) && check_n(num[3]) {
            return Some(String::from_iter(num));
        }
        return None;
    }
    None
}

fn check_n(c: char) -> bool {
    c != '1' && c != '0'
}
