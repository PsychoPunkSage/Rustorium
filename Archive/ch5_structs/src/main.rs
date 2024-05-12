#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 100,
        active: false,
    };

    // WRONG WAY
    // let mut user2 = User{
    //     String::from("user2"),
    //     String::from("user2@example.com"),
    //     0,
    //     false,
    // };

    let name = user1.username;
    user1.username = String::from("AP");

    let user2 = make_user(String::from("APPA"), String::from("ap@ap.com"));
    println!("{:?}", user2);

    // Importing data from other user
    let user3 = User {
        username: String::from("user3"),
        email: String::from("user3@example.com"),
        ..user2 // Importing remaining data from `User2`
    };

    println!("{:?}", user3);

    // TUPLE Structs
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area: {} px", find_area_of_rect(rect));
}

fn make_user(username: String, email: String) -> User {
    User {
        username, // shorthand notation
        email,
        sign_in_count: 0,
        active: false,
    }
}

fn find_area_of_rect(dimensions: Rectangle) -> u32 {
    dimensions.height * dimensions.width
}
