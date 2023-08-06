use std::io;
use rand::Rng;

fn main() {
    println!("|-------------------|");
    println!("|   Guessing Game   |");
    println!("|-------------------|");

    println!();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Input your guess (1-100)");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess was {guess}");
    println!("The secret number was {secret_number}");
}