use std::io;
// use str::repeat;
use rand::prelude::*;

fn main() {
    let upper: u32 = 100;
    let number = rand::thread_rng().gen_range(1..upper);
    higher_or_lower(number, upper);

    /*
    let number = rand::random::<f64>();
    println!("The random number is: {number}");

    let number = thread_rng().gen_range(1..11);
    println!("The random number is: {number}");

    let mut buffer = String::new();
    // println!("Hello guy, type your first name: ");
    println!("Hello guy, type in an integer: ");
    io::stdin().read_line(&mut buffer);
    // println!("buffer is: {buffer}");
    let number = buffer.trim().parse::<i32>().unwrap();
    println!("{number} + 1 is: {}", number + 1);
    
    */

}

fn higher_or_lower(number: u32, upper: u32) {
    println!("Guess a number between 1 and {upper}: ");

    let mut count: usize = 0;
    loop {
        count += 1;
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input line.");
        let guess = guess.trim().parse::<u32>().expect("Failed to parse the guess.");

        if guess > number {
            // println!("Your guess of: {guess} is too high");
            println!("too high, guess again")
        } else if guess < number {
            // println!("Your guess of {guess} is too low");
            println!("too low, guess again");
        } else {
            break
        }
    }

    let stars = "\u{1F320}".repeat(count);
    println!("\n{stars}");
    println!("You guessed the correct number! {number}");
    println!("It only took you {count} guesses.");
}