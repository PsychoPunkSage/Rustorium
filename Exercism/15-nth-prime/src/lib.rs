pub fn nth(n: u32) -> u32 {
    let mut ans: Vec<u32> = vec![2];
    let mut num: u32 = 3;
    (0..n).for_each(|_| {
        let nprime = next_prime(num);
        num = nprime + 1;
        ans.push(nprime);
    });

    *ans.last().unwrap()
}

fn next_prime(start: u32) -> u32 {
    let num = start;
    let is_prime = !(2..=(num / 2 + 1)).any(|i| num % i == 0);

    if is_prime {
        num
    } else {
        next_prime(num + 1)
    }
}
