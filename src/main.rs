extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("The number guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please enter your guess number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error occurred while reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed number is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations, you guessed it right!");
                break;
            }
        }
    }
}
