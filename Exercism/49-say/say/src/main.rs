const UNITS: [&'static str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&'static str; 10] = [
    "_", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

pub fn encode(n: u64) -> String {
    if n == 0 {
        "zero".to_string()
    } else {
        do_encode(n, "")
    }
}

fn do_encode(n: u64, fill_char: &str) -> String {
    match n {
        0 => "".to_string(),
        n if n < 20 => fill_char.to_string() + UNITS[n as usize],
        n if n < 100 => fill_char.to_string() + TENS[(n / 10) as usize] + &do_encode(n % 10, "-"),
        n if n < 1_000 => {
            fill_char.to_string() + &do_encode(n / 100, "") + " hundred" + &do_encode(n % 100, " ")
        }
        n if n < 1_000_000 => {
            fill_char.to_string()
                + &do_encode(n / 1_000, "")
                + " thousand"
                + &do_encode(n % 1_000, " ")
        }
        n if n < 1_000_000_000 => {
            fill_char.to_string()
                + &do_encode(n / 1_000_000, "")
                + " million"
                + &do_encode(n % 1_000_000, " ")
        }
        n if n < 1_000_000_000_000 => {
            fill_char.to_string()
                + &do_encode(n / 1_000_000_000, "")
                + " billion"
                + &do_encode(n % 1_000_000_000, " ")
        }
        n if n < 1_000_000_000_000_000 => {
            fill_char.to_string()
                + &do_encode(n / 1_000_000_000_000, "")
                + " trillion"
                + &do_encode(n % 1_000_000_000_000, " ")
        }
        n if n < 1_000_000_000_000_000_000 => {
            fill_char.to_string()
                + &do_encode(n / 1_000_000_000_000_000, "")
                + " quadrillion"
                + &do_encode(n % 1_000_000_000_000_000, " ")
        }
        _ => {
            fill_char.to_string()
                + &do_encode(n / 1_000_000_000_000_000_000, "")
                + " quintillion"
                + &do_encode(n % 1_000_000_000_000_000_000, " ")
        }
    }
}
