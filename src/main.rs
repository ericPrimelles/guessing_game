use std::io;
fn main() {
    println!("Welcome to the 2024 World Guessing Cup!");
    println!("Input your Guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You've guessed {}", guess);
}
