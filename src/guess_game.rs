use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    // println!("Your secret number is {secret_number}");

    loop {
        let mut guess: String = String::new();
        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = guess.trim().parse().expect("please type a number!");

        println!("You guessed: {guess}");
        if check(secret_number, guess) {
            break;
        }

    }
}

fn check(secret_number: u32, guess: u32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("Yes, you win man!");
            return true
        }
    }
    // implicit return using expression instead of a statement
    // no ";" at the end of the line
    false
}
