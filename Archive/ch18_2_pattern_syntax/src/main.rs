struct Point {
    x: i32,
    y: i32,
}
struct ThreeDPoint {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Text {
    Hello { id: i32 },
}

fn main() {
    let x = 10;
    match x {
        1 | 2 => println!("one or two"),
        3..=8 => println!("three"),
        // 'a'..='m' => println!("alphabets"),
        _ => println!("anything"),
    }

    let y = Some(10);
    match y {
        Some(10) => println!("ten"),
        Some(y /*Shadowing previously defined `y`*/) => println!("Matched; y = {}", y),
        _ => println!("anything"),
    }

    // Destructuring structs
    let p = Point { x: 10, y: 19 };
    let Point { x: a, y: b } = p;
    assert_eq!(a, 10);
    assert_eq!(b, 19);

    match p {
        Point { x, y: 0 } => println!("On x-axis"),
        Point { x: 0, y } => println!("On y-axis"),
        Point { x, y } => println!("Random Point"),
    }

    // Destructuring Enum
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move: x: {}, y: {}", x, y),
        Message::Write(s) => println!("Write: {}", s),
        Message::ChangeColor(r, g, b) => println!("Change color: r: {}, g: {}, b: {}", r, g, b),
    }

    // Range-syntax in Point.
    let origin = ThreeDPoint { x: 0, y: 0, z: 0 };
    match origin {
        ThreeDPoint { x, .. } => println!("Write: {}", x),
    }

    let num_tup = (1, 2, 3, 4, 5, 6, 7, 8);
    match num_tup {
        (first, .., last) => {
            println!("Some number: {}, {}", first, last);
        }
    }

    // Match Guards
    let num = Some(10);
    let y = 12;
    match num {
        Some(x) if x < 10 => println!("less than 10"),
        Some(x) if x > 10 && x < 20 => println!("greater than 10, less than 20"),
        Some(x) if x == y => println!("Matched"), // Using `outside` var inside.
        Some(x) => println!("x = {}", x),
        _ => println!("None"),
    }

    // Matching with Option<T>
    let x = 10;

    match x {
        n if n < 10 => println!("less than 10"),
        n if n > 10 => println!("greater than 10"),
        n => println!("x = {}", n),
    }

    //
    let text = Text::Hello { id: 19 };
    match text {
        Text::Hello { id: id_var @ 3..=9 } => println!("Found id in range {}", id_var),
        Text::Hello {
            id: id_var @ 10..=17,
        } => println!("Found id in range {}", id_var),
        Text::Hello { id } => println!("Found id in range {}", id),
    }
}
