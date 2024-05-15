struct Point<T, U> {
    X: T,
    Y: U,
}

// Generic Impl block
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.X
    }

    fn y(&self) -> &U {
        &self.Y
    }
}

// Concrete Impl block
impl<U> Point<f64, U> {
    fn y_corr(&self) -> f64 {
        self.Y
    }
}

fn main() {
    let nums = vec![1, 20, 98301, 46, 9, 10];
    println!("{}", get_largest_val(nums));
    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    println!("{}", get_largest_val(chars));

    // Struct with generic
    let p1 = Point { X: 0, Y: 10 };
    let p2 = Point { X: 12.89, Y: 10.10 };
    let p2 = Point { X: 12.89, Y: 10 };
}

fn get_largest_val<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for i in list.iter() {
        if *i > largest {
            largest = *i;
        }
    }
    return largest;
}
