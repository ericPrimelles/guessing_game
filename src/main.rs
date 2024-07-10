use std::io;
use rand::Rng;
fn main() {
    println!("Welcome to the 2024 World Guessing Cup!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Input your Guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You've guessed {}", guess);
    println!("The secret Number is {secret_number}")
}
