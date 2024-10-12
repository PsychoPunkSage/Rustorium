pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code = &self.0;
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

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
