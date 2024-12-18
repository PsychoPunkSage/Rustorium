#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if string_digits.chars().any(|c| !c.is_numeric()) {
        return Err(Error::InvalidDigit(
            string_digits
                .chars()
                .filter(|c| !c.is_numeric())
                .collect::<Vec<_>>()[0],
        ));
    }

    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }

    let data = string_digits.chars().collect::<Vec<char>>();

    Ok((0..=(data.len() - span))
        .map(|i| {
            let mut prod = 1;
            for j in 0..span {
                prod *= data[i + j].to_digit(10).unwrap();
            }
            println!("prod:> {}", prod);
            prod
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .max()
        .unwrap() as u64)
}
