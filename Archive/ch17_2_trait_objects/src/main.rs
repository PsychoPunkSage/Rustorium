use ch17_2_trait_objects::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
// implements `Draw`
impl Draw for SelectBox {
    fn draw(&self) {
        println!("Select obj DONE")
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            // Any DS that implements `Draw`
            // Box::new(String::from("Hi")) ==> Doesn't implement `Draw`
            Box::new(SelectBox {
                width: 10,
                height: 13,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}
