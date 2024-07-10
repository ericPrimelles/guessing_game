use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to the 2024 World Guessing Cup!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Input your Guess");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let mut guess_int : i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number guess again!");
                continue;
            }
        };
        println!("You've guessed {}", guess);
        //println!("The secret Number is {secret_number}");
        match guess_int.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You Win!");
                break;

            }
        }

    }

}
