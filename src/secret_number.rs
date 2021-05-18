use std::io;
use rand::prelude::*;

pub fn play() {
    let number = thread_rng().gen_range(1..101);
    
    println!("Try to guess my secret number:");
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read input line");
        let proposal: i32 = buffer.trim().parse().unwrap();
        if proposal == number {
            println!("You have guessed the number {}!", number);
            break;
        } else if proposal > number {
            println!("The number {} is too high! Try again:", proposal);
        } else {
            println!("The number {} is too low! Try again:", proposal);
        }
    }   
}