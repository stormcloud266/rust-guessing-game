use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // clear the terminal
    print!("{esc}c", esc = 27 as char);

    println!("Guess the number!");

    // create a number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // create loop so player can keep guessing until they guess correctly
    loop {
        println!("Please input your guess.");

        // get guess from user input and assign to variable
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read guess");

        // parse input into integer, start loop over if it can't be parsed
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // check guess and break loop if correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
