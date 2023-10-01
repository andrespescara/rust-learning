use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // rand bewteen 1 and 100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Input your guess.");

        // variable String
        let mut guess = String::new();

        // fill String with value
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Cast guess as number, saves on numeric value, otherwise it loops back
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // compares guess with rand, both have to be the same type
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small :("),
            Ordering::Greater => println!("Too Big :("),
            Ordering::Equal => {
                println!("Matched!");
                break;
            }
        }
    }
}
