//! Guessing game implementation

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Returns an integer from user input and panics if the input is invalid
fn get_input() -> i32 {
    println!("Please input your guess");

    // Get user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input as an integer
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid entry."),
    }
}

/// The main function that runs the guessing game
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop until the user guesses the correct number
    loop {
        let guess = get_input();
        print!("You guessed: {}. ", guess);

        // Compare the guess to the secret number and print the appropriate message
        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("That is correct!");
                break;
            }
            Ordering::Greater => println!("You're guess is too low."),
            Ordering::Less => println!("You're guess is too high."),
        }
    }
}
