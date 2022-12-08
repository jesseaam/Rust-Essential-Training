// CHALLENGE
// Implement error handling for the higher or lower guessing game
// If the input is invalid, print a message and let the user guess again
// Use your own solution to the previous challenge as a starting point

use std::io;
use rand::prelude::*;

fn main() {
    let max_num = 11;
    let secret_number = rand::thread_rng().gen_range(1..max_num);

    println!("\nI'm thinking of a number between 1 and {max_num}");
    println!("Guess the number:");
    loop {
        let mut buffer = String::new();
        let guess = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(value) => value, // success
                Err(_) => {
                    println!("\nFailed to parse input. Guess again.");
                    continue
                }
            }
            Err(_) => {
                println!("\nFailed to read input. Guess again.");
                continue
            }
        };

        if guess > secret_number {
            println!("\n{guess} is too high! Guess lower:");
        } else if guess < secret_number {
            println!("\n{guess} is too low! Guess higher:");
        } else {
            println!("\nYou got it! The secret number was {secret_number}");
            break;
        }
    } 
} // fn main


/*
// Propogating errors
use std::fs;
use std::io;

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    // let mut s1 = match fs::read_to_string(f1) {
    //     Ok(s) => s,
    //     Err(e) => return Err(e)
    // };
    let mut s1 = fs::read_to_string(f1)?;
    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e)
    };
    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

fn main() {
    // let result = read_and_combine("planet.txt", "dwarf_planets.txt");
    let result = read_and_combine("planets.txt", "dwarf_planets.txt");
    match result {
        Ok(s) => println!("result is...\n{}", s),
        Err(e) => println!("There was an error: {}", e)
    };
}



// Matching Result<T, E> to recover from errors

use std::fs;
use std::io;

fn main() {
    let result = fs::read_to_string("the_ultimate_question.txt");

    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found."),
            io::ErrorKind::PermissionDenied => String::from("Permission denied."),
            _ => panic!("Another type of error: {error:?}")

        }   
     };
    println!("contents is: {:?}", contents);
}




// Result<T, E> enum

use std::fs;

fn main() {
    let contents = fs::read_to_string("answer_to_the_ultimate_question.txt").unwrap();
    // let contents = fs::read_to_string("the_ultimate_question.txt").expect("Nobody knows the ultimate question!");
    println!("contents is: {:?}", contents);
}




// Unrecoverable errors

fn main() {
    // panic!("Houston, we have a problem.")

    let countdown = [5, 4, 3, 2, 1, 0];
    for count in countdown.iter() {
        println!("T-minus {count}");
        let x = 1 / count; // this won't end well
    }
}








        let mut buffer = String::new();
        let guess = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(value) => value, // success
                Err(_) => {
                    println!("\nFailed to parse input. Guess again.");
                    continue
                }
            }
            Err(_) => {
                println!("\nFailed to read input. Guess again.");
                continue

            }
        };

*/

