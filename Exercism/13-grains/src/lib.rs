pub fn square(s: u32) -> u64 {
    (2 as u64).pow(s - 1)
}

pub fn total() -> u64 {
    ((2 as u128).pow(64) - 1) as u64
}
