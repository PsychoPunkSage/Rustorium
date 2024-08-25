mod lib;
use lib::build_proverb;

fn main() {
    let input = &["nail", "shoe", "horse"];
    let output = build_proverb(input);
    let expected: String = [
        "For want of a nail the shoe was lost.",
        "For want of a shoe the horse was lost.",
        "And all for the want of a nail.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}
