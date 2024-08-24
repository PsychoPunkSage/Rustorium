pub fn factors(n: u64) -> Vec<u64> {
    // let mut ans = Vec::new();
    // let mut divisors = 2..;
    // while n > 1 {
    //     let x = divisors.next().unwrap();
    //     while n % x == 0 {
    //         n /= x;
    //         ans.push(x);
    //     }
    // }
    // ans

    (2..=n)
        .find(|i| n % i == 0)
        .map_or_else(Vec::new, |i| [vec![i], factors(n / i)].concat())
}
