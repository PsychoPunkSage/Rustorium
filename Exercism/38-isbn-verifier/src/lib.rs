/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let v = isbn
        .chars()
        .filter(|&c| c.is_numeric() || c == 'X')
        .collect::<Vec<char>>();

    if v.len() == 10 {
        println!("Valid ISBN number Structure\n{:?}", v);
        let s = (0..=9)
            .map(|i| {
                if v[i] == 'X' {
                    println!("{}X{} = {}", v[i], 10 - i, 10 * (10 - i) as u32);
                    10 * (10 - i) as u32
                } else {
                    println!(
                        "{}X{} = {}",
                        v[i],
                        10 - i,
                        v[i].to_digit(10).unwrap() * (10 - i) as u32
                    );
                    v[i].to_digit(10).unwrap() * (10 - i) as u32
                }
            })
            .collect::<Vec<u32>>()
            .into_iter()
            .sum::<u32>();
        println!("{}", s);
        return s % 11 == 0;
    }
    false
}
