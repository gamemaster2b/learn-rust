use rand::prelude::*;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let secret_number: u8 = thread_rng().gen_range(1..=100);

    println!("Guess a number from 1 to 100.");

    loop {
        print!("\nGuess: ");
        io::stdout().flush().expect("flush failed!");

        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read line: {:?}", e);
                continue;
            }
        };

        let guess = match guess.trim().parse::<u8>() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Please enter a number from 1 to 100! {:?}", e);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Congratulations! You guessed the number: 🥰");
                break;
            }
            Ordering::Greater => println!("It won't fit: 🥵"),
            Ordering::Less => println!("Size doesn't matter: 🥶"),
        }
    }
}
