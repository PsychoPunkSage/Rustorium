use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let symbols = [
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];

        Roman(
            symbols
                .iter()
                .map(|(symbol, val)| {
                    let mut count = 0;
                    let mut roman = String::new();
                    while num >= *val && count <= 3 {
                        roman.push_str(symbol);
                        num -= *val;
                        count += 1;
                    }
                    roman
                })
                .collect::<String>(),
        )

        // Roman(roman)
    }
}
