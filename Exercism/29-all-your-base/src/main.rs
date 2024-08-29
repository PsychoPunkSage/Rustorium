use allyourbase as ayb;

fn main() {
    let input_base = 2;
    let input_digits = &[1, 0, 1, 0, 1, 0];
    let output_base = 10;
    let output_digits = vec![4, 2];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}
