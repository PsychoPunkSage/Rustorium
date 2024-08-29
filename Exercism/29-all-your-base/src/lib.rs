use std::result;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base == 0 || from_base == 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base == 0 || to_base == 1 {
        return Err(Error::InvalidOutputBase);
    }

    let arr = number
        .iter()
        .filter(|&d| d >= &from_base)
        .collect::<Vec<&u32>>();

    if arr.len() > 0 {
        return Err(Error::InvalidDigit(arr[0].clone()));
    }

    let mut sum = number
        .iter()
        .enumerate()
        .map(|(i, &d)| d * (from_base.pow((number.len() - 1 - i) as u32)))
        .collect::<Vec<u32>>()
        .into_iter()
        .sum::<u32>();

    println!("num {}", sum);

    let mut result = Vec::new();
    while sum > 0 {
        let digit = sum % to_base;
        sum /= to_base;
        result.insert(0, digit);
    }
    println!("num::> {:?}", result);

    if result.len() == 0 {
        result.push(0);
    }
    return Ok(result);
}
