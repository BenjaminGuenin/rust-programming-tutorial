use std::io;

fn main() {
    println!("Guess the number!");
    println!("Enter your input...");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess);
    println!("You guessed {}", guess);
}