use std::io;

fn main() {
    println!("|-------------------|");
    println!("|   Guessing Game   |");
    println!("|-------------------|");

    println!();

    println!("Input your guess (0-100)");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess was {guess}")
}