// Externals
extern crate rand;

// Usings
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);
    let mut number_of_guesses: u32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("To large"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        number_of_guesses = number_of_guesses + 1;
    }
    println!("It took {} guesses to win.", number_of_guesses);
}
