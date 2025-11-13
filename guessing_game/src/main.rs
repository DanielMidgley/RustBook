use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    loop {
        println!("Guess a number.");

        // Create a variable
        let mut guess = String::new();

        // Get input from command line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Guess: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        /*
        let x = 3;
        let y = 2;
        println!("x + y = {}", x + y);

        let z = x + y;
        println!("x + y = {z}")
        */

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }

}
