// lets Rust know we’ll be using the rand crate as an external dependency
// This also does the equivalent of calling `use rand`
extern crate rand;

use std::io;
use std::cmp::Ordering;

// The Rng trait defines methods that random number generators implement
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // The rand::thread_rng function will give us the particular random number generator;
    // one that is local to the current thread of execution and seeded by the operating system
    // The gen_range method is defined by the Rng trait that we brought into scope with the use rand::Rng statement.
    let secret_number = rand::thread_rng().gen_range(1, 101); // number in [0, 101)

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing: lets us reuse the guess variable name 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue // The underscore, _, is a catchall value
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
