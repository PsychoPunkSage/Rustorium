// NOOB Way \\

// pub fn encrypt(input: &str) -> String {
//     let vecc = input
//         .to_ascii_lowercase()
//         .chars()
//         .filter(|c| c.is_alphanumeric())
//         .collect::<Vec<char>>();
//     if vecc.is_empty() {
//         return "".into();
//     }

//     let (_, r) = get_col_row(vecc.len());

//     let mut data = vecc
//         .chunks(r as usize)
//         .map(|chunk| chunk.iter().map(|c| *c).collect::<Vec<char>>())
//         .collect::<Vec<Vec<char>>>();

//     let len_diff = data[0].len() - data.last().unwrap().len();

//     if len_diff > 0 {
//         data.last_mut()
//             .unwrap()
//             .extend(std::iter::repeat(' ').take(len_diff));
//     }

//     let mut ans = String::new();

//     for i in 0..data[0].len() {
//         for dat in data.iter() {
//             ans.push(dat[i]);
//         }
//         if i != data[0].len() - 1 {
//             ans.push(' ');
//         }
//     }

//     return ans;
// }

// fn get_col_row(len: usize) -> (usize, usize) {
//     let sqrt_len = (len as f64).sqrt().ceil() as usize;

//     if sqrt_len * sqrt_len == len {
//         (sqrt_len, sqrt_len)
//     } else {
//         (sqrt_len + 1, sqrt_len)
//     }
// }

// PRO Way \\
pub fn encrypt(input: &str) -> String {
    let data = input
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<_>>();
    let c = (data.len() as f32).sqrt().ceil() as usize;
    (0..c)
        .map(|i| {
            data.chunks(c)
                .map(|t| t.get(i).unwrap_or(&' '))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
