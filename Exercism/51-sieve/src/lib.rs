pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    for num in 2..upper_bound + 1 {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}

fn is_prime(num: u64) -> bool {
    !(2..=num / 2)
        .map(|n| if num % n == 0 { false } else { true })
        .collect::<Vec<bool>>()
        .iter()
        .any(|a| !*a)
}
