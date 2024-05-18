#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter::> Takes ownership of vector.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

/////////////////////
// Custom Iterator //
/////////////////////
#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // Associated types

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 10 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec!['a', 'b', 'c', 'd', 'e'];

    for value in v1.iter() {
        println!("Got: {}", value);
    }

    // SUM
    let summ: u32 = v1.iter().sum();
    println!("Sum: {}", summ);

    let v: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(vec![2, 3, 4, 5, 6], v);

    // Shoes example
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );

    // Custom Iterator
    let mut counter = Counter::new();
    for i in 1..11 {
        assert_eq!(counter.next(), Some(i));
    }
    assert_eq!(counter.next(), None);
    println!("{:#?}", counter.next());
    println!("{:#?}", counter);

    // other iterators
    let sums: u32 = Counter::new()
        .zip(Counter::new().skip(2)) // Joins to iterators in pairs
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("{:#?}", sums);

    // println!("{:?}", Counter::new().skip(2));
    // println!("{:?}", Counter::new().zip(Counter::new().skip(2)));
    // println!(
    //     "{:?}",
    //     Counter::new()
    //         .zip(Counter::new().skip(2))
    //         .map(|(a, b)| a * b)
    // );

    // 1 2 3 4 5 6 7  8  9 10
    // 3 4 5 6 7 8 9 10 11 12
    // 3  15 24  48 63
}
