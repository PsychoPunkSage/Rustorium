pub fn number(user_number: &str) -> Option<String> {
    let num = user_number
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<Vec<char>>();

    if (num.len() == 11 && num[0] == '1' && check_n(num[1]) && check_n(num[4]))
        || (num.len() == 10 && num[0] != '1' && check_n(num[0]) && check_n(num[3]))
    {
        if num.len() == 11 {
            return Some(String::from_iter(&num[1..]));
        }

        return Some(String::from_iter(num));
    }
    None
}

fn check_n(c: char) -> bool {
    c != '1' && c != '0'
}
