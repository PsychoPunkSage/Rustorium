fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // Another representation
    let v2 = vec![1, 2, 3, 4];
    println!("{:?}", v2);
    println!("{}", &v2[2]);

    // Safer access of elements in Vector ::> ise "".get(index)""
    match v.get(3) {
        Some(ele) => println!("4th element {}", ele),
        None => println!("No third element"),
    }

    // Parsing a vector
    for i in a {
        println!("{}", i);
    }

    for ji in &mut a.iter() {
        println!("{}", *ji + 50);
    }

    for j in a.iter() {
        println!("{}", *j);
    }

    for (k, l) in a.iter().enumerate() {
        // k -> index || l -> element reference
        println!("{} {}", k, l);
    }
}
