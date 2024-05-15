// At runtime RUST will auto-generate / compile Generic for different cases.
// i.e. Point<i32, i32>, Point<&str, char> Point<f64, f64> Point<f64, i32>
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

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            X: self.X,
            Y: other.Y,
        }
    }
}

// Concrete Impl block
impl<U> Point<U, f64> {
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
    let p2 = Point { X: "Hello", Y: 'c' };
    let p3 = Point { X: 12.89, Y: 10.10 };
    let p4 = Point { X: 12.89, Y: 10 };

    let p_t = p1.mixup(p2);
    println!("V_mix: {} || Y_mix: {}", p_t.X, p_t.Y);
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
