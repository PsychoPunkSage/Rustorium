mod lib;
use lib::solve;

fn main() {
    let answer = solve("I + BB == ILL");
    let expected = [('I', 1), ('B', 9), ('L', 0)].into_iter().collect();
    assert_eq!(answer, Some(expected));
}
