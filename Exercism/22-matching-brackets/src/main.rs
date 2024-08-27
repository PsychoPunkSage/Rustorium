mod lib;
use lib::brackets_are_balanced;

fn main() {
    assert!(!brackets_are_balanced("{}["));
}
