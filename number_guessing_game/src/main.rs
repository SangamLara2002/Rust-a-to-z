use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to the guessing game!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // 2. Generate a random number between 1 and 100.
    let mut remaining_guesses = 5;
    // print!("{}", secret_number);
    loop {
        if remaining_guesses == 0 {
            println!("You lose!");
            break;
        }
        println!("Please input your guess.");
        let mut guess = String::new(); // 1. Create a mutable variable to store the user's guess because it will change and variables are immutable by default.
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

        remaining_guesses -= 1;
        println!("You have {} guesses left.", remaining_guesses);
    }
}
