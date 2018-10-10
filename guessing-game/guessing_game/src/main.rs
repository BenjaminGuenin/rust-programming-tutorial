extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop
    {
        println!("Enter your input...");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Switching from an expect call to a match expression is how you generally move from crashing on an error to handling the error
        let guess: u32 = match guess.trim().parse()
        {
            // Remember that parse returns a Result type and Result is an enum that has the variants Ok or Err
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too small!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }

        println!("You guessed {}", guess);
    }
}