pub fn is_armstrong_number(num: u32) -> bool {
    num == num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num.to_string().len() as u32))
        .sum()
}
