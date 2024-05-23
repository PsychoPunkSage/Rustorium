use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let sec_num = rand::thread_rng().gen_range(1..100);

    println!("Welcome to Guessing Game");
    println!("Enter the Guess number:");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        println!("Your Guess value is: {}", guess);

        match guess.cmp(&sec_num) {
            Ordering::Equal => {
                println!("{}", "You Win!!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Less => println!("{}", "Too Small".red()),
        }
    }
    println!("Secret number: {}", sec_num);
}
