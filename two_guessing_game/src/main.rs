use std::{cmp::Ordering, io}; // io lib in scope 

// Random Library
// to add deps "rand" package => add `deps = "version"` in `Cargo.toml` => cargo build
// [dependencies]
// rand = "0.5.5"
use rand::{Rand, Rng}; 


fn main() {
    // intro lines print
    println!("Guess the Number !!!"); // like python/c
    println!("Input Your Guess:");

    // variable to store stuff
    // String, A type is Rust Standard library, utf-8, growable string
    // new() is associative func. static method, create empty string
    // Variables in Rust are DEFAULT IMMUTABLE, to make them mutable, use mut keyword
    let mut guess = String::new(); // like java

    // io lib in scope 
    // use std::io; // io lib in scope 
    // .read_line method to read line
    // Result cases to 1. Ok() & 2. Err()
    io::stdin() // like java
        .read_line(&mut guess) 
        .expect("Failed to Read Line"); // iff err comes, .expect() crash program, and display message
        
    // .trim() remove whitespaces
    // .parse() helps to parse
    let guess: u32 = guess.trim().parse()
    .expect("Failed to Read Line");// error handling strict by language

    println!("You Guessed: {}",guess); // like c
    // Guess the Number !!!
    // Input Your Guess:
    // 12
    // You Guessed: 12

    // Now Random Check is Left

    // // Random Library
    // // to add deps "rand" package => add `deps = "version"` in `Cargo.toml` => cargo build
    // // [dependencies]
    // // rand = "0.5.5"
    // use rand::{Rand, Rng}; 
    let secret_nos = rand::thread_rng().gen_range(1,101); // lower limit is inclusive, upper limit is exclusive 
    println!("Actual Number: {}", secret_nos);

    // cmp::Ordering library
    match guess.cmp(&secret_nos){
        Ordering::Equal => print!("YOU WIN !!!"),
        Ordering::Less => print!("TOO SMALL !!!"),
        Ordering::Greater => print!("TOO BIG !!!")
    }


}

// Guess the Number !!!
// Input Your Guess:
// 2
// You Guessed: 2
// Actual Number: 2
// YOU WIN !!!