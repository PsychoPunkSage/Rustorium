// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use core::str;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let arr = input.split("\n").collect::<Vec<&str>>();
    if arr.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(arr.len()));
    }

    if arr[0].len() % 3 != 0 {
        return Err(Error::InvalidColumnCount(arr[0].len()));
    }

    let mut result = String::new();

    for (i, chunk) in arr.chunks(4).enumerate() {
        let arr_1 = chunk_string(chunk[0], 3);
        let arr_2 = chunk_string(chunk[1], 3);
        let arr_3 = chunk_string(chunk[2], 3);
        let mut tmp_res = String::new();
        arr_1
            .iter()
            .zip(arr_2.iter())
            .zip(arr_3.iter())
            .for_each(|((a, b), c)| {
                let mut tmp = String::new();
                // println!("{}\n{}\n{}\n\n", a, b, c);
                let a_vec = a.chars().collect::<Vec<char>>();
                let b_vec = b.chars().collect::<Vec<char>>();
                let c_vec = c.chars().collect::<Vec<char>>();

                // println!("{:?}", a_vec);

                if a_vec[1] == '_'
                    && b_vec[0] == '|'
                    && b_vec[1] == ' '
                    && b_vec[2] == '|'
                    && c_vec[0] == '|'
                    && c_vec[1] == '_'
                    && c_vec[2] == '|'
                {
                    tmp.push('0');
                }

                if a == &"   "
                    && b_vec[0] == ' '
                    && b_vec[1] == ' '
                    && b_vec[2] == '|'
                    && c_vec[0] == ' '
                    && c_vec[1] == ' '
                    && c_vec[2] == '|'
                {
                    tmp.push('1');
                }

                if a_vec[0] == ' '
                    && a_vec[1] == '_'
                    && a_vec[2] == ' '
                    && b_vec[0] == ' '
                    && b_vec[1] == '_'
                    && b_vec[2] == '|'
                    && c_vec[0] == '|'
                    && c_vec[1] == '_'
                    && c_vec[2] == ' '
                {
                    tmp.push('2');
                }

                if a == &"   "
                    && b_vec[0] == '|'
                    && b_vec[1] == '_'
                    && b_vec[2] == '|'
                    && c_vec[0] == ' '
                    && c_vec[1] == ' '
                    && c_vec[2] == '|'
                {
                    tmp.push('4');
                }

                if a_vec[0] == ' '
                    && a_vec[1] == '_'
                    && a_vec[2] == ' '
                    && b_vec[0] == ' '
                    && b_vec[1] == '_'
                    && b_vec[2] == '|'
                    && c_vec[0] == ' '
                    && c_vec[1] == '_'
                    && c_vec[2] == '|'
                {
                    tmp.push('3');
                }

                if a_vec[0] == ' '
                    && a_vec[1] == '_'
                    && a_vec[2] == ' '
                    && b_vec[0] == '|'
                    && b_vec[1] == '_'
                    && b_vec[2] == ' '
                    && c_vec[0] == ' '
                    && c_vec[1] == '_'
                    && c_vec[2] == '|'
                {
                    tmp.push('5');
                }

                if a_vec[0] == ' '
                    && a_vec[1] == '_'
                    && a_vec[2] == ' '
                    && b_vec[0] == '|'
                    && b_vec[1] == '_'
                    && b_vec[2] == ' '
                    && c_vec[0] == '|'
                    && c_vec[1] == '_'
                    && c_vec[2] == '|'
                {
                    tmp.push('6');
                }

                if a_vec[0] == ' '
                    && a_vec[1] == '_'
                    && a_vec[2] == ' '
                    && b_vec[0] == ' '
                    && b_vec[1] == ' '
                    && b_vec[2] == '|'
                    && c_vec[0] == ' '
                    && c_vec[1] == ' '
                    && c_vec[2] == '|'
                {
                    tmp.push('7');
                }

                if a_vec[0] == ' '
                    && a_vec[1] == '_'
                    && a_vec[2] == ' '
                    && b_vec[0] == '|'
                    && b_vec[1] == '_'
                    && b_vec[2] == '|'
                    && c_vec[0] == '|'
                    && c_vec[1] == '_'
                    && c_vec[2] == '|'
                {
                    tmp.push('8');
                }

                if a_vec[0] == ' '
                    && a_vec[1] == '_'
                    && a_vec[2] == ' '
                    && b_vec[0] == '|'
                    && b_vec[1] == '_'
                    && b_vec[2] == '|'
                    && c_vec[0] == ' '
                    && c_vec[1] == '_'
                    && c_vec[2] == '|'
                {
                    tmp.push('9');
                }

                if tmp.is_empty() {
                    tmp.push('?');
                }

                tmp_res.push_str(&tmp);
            });

        if i > 0 {
            result.push(',');
        }
        result.push_str(&tmp_res);
    }

    Ok(result)
}

fn chunk_string(s: &str, chunk_size: usize) -> Vec<&str> {
    s.as_bytes()
        .chunks(chunk_size)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
}
