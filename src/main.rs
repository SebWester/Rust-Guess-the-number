use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // Type String
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Konvertera guess från String till Int --> u32
    // trim() --> Ta bort "vita" tecken = Mellanslag/Radbrytning
    // parse() --> Omvandlar strängen till datatyp som anges
    // expect() --> Felmeddelande som körs om parse() returnerar ett fel
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!!"),
        Ordering::Equal => println!("That's right!"),
    }
}
