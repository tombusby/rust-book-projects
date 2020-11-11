use rand::Rng;
use std::cmp::Ordering;
use std::io;

const LOWER_BOUND: u32 = 1;
const UPPER_BOUND: u32 = 10000;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(LOWER_BOUND, UPPER_BOUND);

    let mut guesses = 0;
    let mut lower_bound = LOWER_BOUND;
    let mut upper_bound = UPPER_BOUND;

    loop {
        println!(
            "Please input your guess. \
                (Lower bound: {}, Upper bound: {}, Midpoint: {})",
            lower_bound,
            upper_bound,
            upper_bound - (upper_bound - lower_bound) / 2
        );

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        guesses += 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if guess > lower_bound {
                    lower_bound = guess;
                }
                println!("Too small!")
            }
            Ordering::Greater => {
                if guess < upper_bound {
                    upper_bound = guess;
                }
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win with {} guesses!", guesses);
                break;
            }
        }
    }
}
