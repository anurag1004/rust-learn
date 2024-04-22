use std::io;

use rand::Rng;
fn main() {
    let mut fname = String::new();
    fname.push('2');
    println!("Hello, {fname}");
    // generate the secret number
    let secret_number = rand::thread_rng().gen_range(1..=200); //[1,200]
    let mut guess = String::new();
    println!("Input your guess: ");
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    guess = guess.trim_end().to_string(); // removes new line from the end
    println!("You guessed {guess} and secrect number {secret_number}")
}
