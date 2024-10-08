#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .into_iter()
        .flat_map(|&val| {
            let mut buffer = Vec::new();
            let mut n = val;
            loop {
                let mut byte = (n & 0x7F) as u8; // 0x7F: 01111111
                n >>= 7;

                if !buffer.is_empty() {
                    // If the buffer is empty.... then the byte about to be appended will be TRAILING bytes, so, must MSB = 0
                    byte |= 0x80; // 10000000
                }

                buffer.push(byte);

                if n == 0 {
                    break;
                }
            }
            buffer.into_iter().rev()
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut numbers = Vec::new();
    let mut current_number: u32 = 0;
    let mut continuation = false;

    for (_, &byte) in bytes.iter().enumerate() {
        // 0x7F: 01111111
        let value = (byte & 0x7F) as u32; // last 7  bits :: exclude MSB
        current_number = (current_number << 7) | value; // Shift left 7 bits and OR with new value
                                                        // BUTWISE OR == addition

        if byte & 0x80 == 0 {
            // i.e. MSB is 0, this is the last byte of the current number
            numbers.push(current_number);
            current_number = 0;
            continuation = false;
        } else {
            continuation = true; // MSB still != 0...
        }
    }

    if continuation {
        return Err(Error::IncompleteNumber);
    }

    Ok(numbers)
}
