use rand::prelude::*;
use std::io;

fn main() {
    print!("Guess a number from 1 to 100: ");

    let screte_number: u8 = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess = match guess.trim().parse::<u8>() {
        Ok(num) => num,
        Err(_) => panic!("Please type a number!"),
    };

    println!(
        "You guessed \"{}\" is {}.",
        guess,
        match guess.eq(&screte_number) {
            true => "right🎆",
            false => "wrong⛔",
        }
    );
}
