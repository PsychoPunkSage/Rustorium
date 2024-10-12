pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> Luhn for T
where
    T: ToString,
{
    fn valid_luhn(&self) -> bool {
        let code = self.to_string();
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
}
