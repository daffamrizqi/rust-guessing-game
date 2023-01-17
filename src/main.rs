use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    // 4
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess!");

        // 1
        let mut guess = String::new();

        // 2
        io::stdin()
            .read_line(&mut guess)
            // 3
            .expect("Failed to read guessing!");

        // 6
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // 5
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
