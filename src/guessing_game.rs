use std::{io::{self, Write}, cmp::Ordering};
use rand::Rng;

pub fn play_game() {
    let number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number! [1, 100]");

    loop {
        print!("\nYour guess: ");
        io::stdout().flush().expect("Couldn't flush.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your input :S");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type a valid number!");
                continue;
            }
        };

        match guess.cmp(&number) { 
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },

        }
    }
}