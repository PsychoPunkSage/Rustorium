/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|c| !c.is_whitespace() && !c.is_digit(10)) {
        return false;
    }

    let series: Vec<u32> = code.chars().rev().filter_map(|c| c.to_digit(10)).collect();

    if series.len() <= 1 {
        return false;
    }

    let sum: u32 = series
        .iter()
        .enumerate()
        .map(|(i, &num)| {
            println!("{} || {}", i, num);
            if i % 2 == 1 {
                let doubled = num * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                num
            }
        })
        .sum();

    sum % 10 == 0
}
