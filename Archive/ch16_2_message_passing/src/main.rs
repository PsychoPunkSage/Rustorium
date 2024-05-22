use std::sync::mpsc; // multi-producer single-consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let msg = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("spawned"),
            String::from("by"),
            String::from("main"),
            String::from("thread"),
            String::from("!"),
        ];
        // tx.send(msg).unwrap();
        for val in msg {
            match tx.send(val) {
                Ok(_) => println!("Message sent"),
                Err(e) => println!("Error: {}", e),
            };
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let msg = vec![
            String::from("1234"),
            String::from("5678"),
            String::from("9101"),
            String::from("9101"),
            String::from("2345"),
            String::from("6789"),
            String::from("0123"),
            String::from("4567"),
        ];
        // tx.send(msg).unwrap();
        for val in msg {
            match tx2.send(val) {
                Ok(_) => println!("Message sent"),
                Err(e) => println!("Error: {}", e),
            };
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let received = rx.recv().unwrap();
    // let received = match rx.recv() {
    //     Ok(msg) => msg,
    //     Err(e) => {
    //         println!("Error: {}", e);
    //         String::from("No message received")
    //     }
    // };
    // println!("Got message: {}", received);

    for received in rx.iter() {
        println!("Got: {}", received);
    }
    /*
    recv()::> Stops main thread exec and awaits for a msg to be sent down the channel.
    try_recv()::> Wont stop main thread exec, instead it will return result immediately.
     */
}
