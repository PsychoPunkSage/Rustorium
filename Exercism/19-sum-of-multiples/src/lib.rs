pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = vec![];

    factors.iter().for_each(|&d| {
        let mut a = d;
        while a < limit && a != 0 {
            if !multiples.contains(&a) {
                multiples.push(a);
            }
            a += d;
        }
    });

    multiples.into_iter().sum()
}

pub fn sum_of_multiples_optimized(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|n| {
            factors
                .iter()
                .filter(|&m| *m > 0)
                .filter(|&o| n % o == 0)
                .sum::<u32>()
                > 0
        })
        .sum()
}
