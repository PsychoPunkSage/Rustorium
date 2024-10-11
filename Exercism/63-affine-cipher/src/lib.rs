/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !coprime(a as u32, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let dataset = plaintext
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() && !c.is_whitespace())
        .collect::<Vec<char>>();
    println!("{:?}", dataset);

    Ok(String::from_iter(
        dataset
            .into_iter()
            .map(|c| {
                if c.is_alphabetic() {
                    let index = (b'a'..=b'z')
                        .into_iter()
                        .position(|a| a == c.to_string().as_bytes()[0])
                        .unwrap();
                    let new_index = (a * (index as i32) + b) % 26;
                    (new_index as u8 + b'a') as char
                } else {
                    c
                }
            })
            .enumerate()
            .map(|(i, c)| {
                if i % 5 == 4 {
                    // after every 5th character
                    format!("{} ", c)
                } else {
                    c.to_string()
                }
            })
            .collect::<Vec<String>>(),
    )
    .trim()
    .to_string())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !coprime(a as u32, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let dataset = ciphertext
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() && !c.is_whitespace())
        .collect::<Vec<char>>();

    Ok(String::from_iter(
        dataset
            .into_iter()
            .map(|c| {
                if c.is_alphabetic() {
                    let index = (b'a'..=b'z')
                        .into_iter()
                        .position(|a| a == c.to_string().as_bytes()[0])
                        .unwrap();

                    let an = mmi(a, 26);
                    let new_index = ((index as i32 - b) * an).rem_euclid(26); // avoid -ve index
                    (new_index as u8 + b'a') as char
                } else {
                    c
                }
            })
            .collect::<Vec<char>>(),
    ))
}

fn coprime(a: u32, b: u32) -> bool {
    gcd(a, b) == 1
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn mmi(a: i32, m: i32) -> i32 {
    for x in 1..m {
        if (a * x) % m == 1 {
            return x;
        }
    }
    panic!("MMI does not exist for a = {} and m = {}", a, m);
}
