/*
- In Enums; we can group many things inside message tags
*/

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKindConcise {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_fn_x() {
        println!("Yeah, I'm here");
    }
}

fn main() {
    let four = IpAddrKind::V4; // Enums are namespaced under its identifier
    let six = IpAddrKind::V6;
    println!("{:#?}", four);
    println!("{:#?}", six);

    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // Concise notation
    let localhost1 = IpAddrKindConcise::V4(String::from("127.0.0.1"));
    println!("{:#?}", localhost1);
    let localhost2 = IpAddrKindConcise::V4(String::from("127.0.0.9"));
    println!("{:#?}", localhost2);

    println!("{:#?}", localhost);

    // OPTION enum
    let some_num = Some(10);
    let some_string = Some(String::from("AP is here"));
    let null_num: Option<i32> = None;

    let x = 19;
    // let y = Some(10);
    let y = None;
    println!("{:#?}", x + y.unwrap_or(2));
}
