use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
    
        // Type String
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // Convert guess from String to Int --> u32
        // trim() --> Remove "white" chars = Spaces/Line breaks
        // parse() --> Converting String to specified type
        // expect() --> Error if parse() returns error
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("That's right!"); 
                break;
            }                
        }
    }
}

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
