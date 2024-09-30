pub fn encode(n: u64) -> String {
    let mut num = n;
    let mut result = String::new();

    let mut sets = 0;

    if num > 0 {
        while num > 0 {
            let hundreds = (num % 1000) as usize;
            num /= 1000;

            let name = get_hundred_name(hundreds as u64);
            let set_name = if hundreds > 0 {
                match sets {
                    0 => "",
                    1 => "thousand",
                    2 => "million",
                    3 => "billion",
                    4 => "trillion",
                    5 => "quadrillion",
                    6 => "quintillion",
                    _ => panic!("Out of scope"),
                }
            } else {
                ""
            };

            result = format!("{} {} {}", name, set_name, result);
            sets += 1;
        }
        return result.trim().to_owned();
    }

    return String::from("zero");
}

fn get_hundred_name(n: u64) -> String {
    let units = vec![
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let teens = [
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

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let hundreds = (n / 100) as usize;
    let ten = ((n % 100) / 10) as usize;
    let unit = (n % 10) as usize;

    let hundred_name = if (1..units.len()).contains(&hundreds) {
        format!("{} hundred ", units[hundreds])
    } else {
        String::new()
    };

    let ten_name = if ten == 1 {
        // i.e. between 0 - 19
        format!("{}{}", teens[unit], "")
    } else if ten > 1 && unit == 0 {
        format!("{}", tens[ten])
    } else if ten > 1 && unit != 0 {
        format!("{}-{}", tens[ten], units[unit])
    } else {
        format!("{}", units[unit])
    };

    hundred_name + &ten_name
}
