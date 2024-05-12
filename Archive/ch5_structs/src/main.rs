#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
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
}

fn make_user(username: String, email: String) -> User {
    User {
        username, // shorthand notation
        email,
        sign_in_count: 0,
        active: false,
    }
}
