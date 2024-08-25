mod lib;
use lib::sum_of_multiples;

fn main() {
    let factors = &[43, 47];
    let limit = 10_000;
    let output = sum_of_multiples(limit, factors);
    let expected = 2_203_160;
    assert_eq!(output, expected);
}
