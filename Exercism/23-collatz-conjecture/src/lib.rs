pub fn collatz(/*mut*/ n: u64) -> Option<u64> {
    // if n == 0 {
    //     return None;
    // }
    // let mut steps = 0;
    // while n != 1 {
    //     if n % 2 == 0 {
    //         n /= 2;
    //     } else {
    //         n = 3 * n + 1;
    //     }
    //     steps += 1;
    // }

    // return Some(steps);

    match n {
        0 => None,
        1 => Some(0),
        _ => {
            if n % 2 == 0 {
                return collatz(n / 2).and_then(|u| Some(u + 1));
            } else {
                return collatz(3 * n + 1).and_then(|u| Some(u + 1));
            }
        }
    }
}
