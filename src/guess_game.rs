use std::io::{self, Write};
use rand::Rng;

pub(crate) fn run() {
    println!("Guess the number (1 to 100)!");

    // use rand to generate random number
    let secret_number = rand::thread_rng().gen_range(1..100);
    let mut attempts = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // add > to the input line
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        attempts += 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win! It took you {} attempts.", attempts);
                break;
            }
        }
    }
}