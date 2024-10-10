fn main() {
    let n = 5;

    for row in 0..n {
        let mut coeff = 1;
        for col in 0..=row {
            print!("{}", coeff);
            coeff = coeff * (row - col) / (col + 1);
        }
        println!();
    }
}

/*
Write a function that generates the first n rows of Pascal's Triangle.

constrain N = 5


ROUGH:
1
11
121
1331
*/
