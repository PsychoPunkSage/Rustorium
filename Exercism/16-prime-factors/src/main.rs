mod lib;
use lib::factors;

fn main() {
    let factors = factors(93_819_012_551);
    let expected = [11, 9_539, 894_119];
    assert_eq!(factors, expected);
}
