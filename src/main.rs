use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("|----------------------------------|");
    println!("|   Guessing Game (self solving)   |");
    println!("|----------------------------------|");

    println!();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guesses_small: Vec<u32> = vec![0];
    let mut guesses_big: Vec<u32> = vec![100];

    let mut guess = 50;

    loop {
        println!("Input your guess (1-100)");

        println!("{guess}");

        println!("Your guess was {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");

                guesses_small.push(guess);
                guess += (get_smallest(&guesses_big) - get_biggest(&guesses_small)) / 2
            },
            Ordering::Greater => {
                println!("Too big!");

                guesses_big.push(guess);
                guess -= (get_smallest(&guesses_big) - get_biggest(&guesses_small)) / 2
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_biggest(vec: &Vec<u32>) -> u32 {
    *vec.iter().max().unwrap()
}

fn get_smallest(vec: &Vec<u32>) -> u32 {
    *vec.iter().min().unwrap()
}