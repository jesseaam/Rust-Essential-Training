use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {

    if env::args().len() < 2 {
        println!("This program requires exactly two arguments: <file path> <search name>")
    }

    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    for (index, line) in fs::read_to_string(&file_path).unwrap().lines().enumerate() {
        if search_name == line {
            println!("{search_name} DID walk on the moon!");
            println!("{search_name} was number {} in the list.", index + 1);
            return;
        }
    }
    println!("{search_name} has not walked on the moon... yet!");
    println!("We will append {search_name} to the end of the file, though.");
    let mut names = fs::OpenOptions::new()
        .append(true)
        .open(file_path)
        .unwrap();
    write!(names, "{search_name}");
    // names.write(search_name.as_bytes());



    /*
    // mutable variable to serve as a reference to the file
    let mut planets = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
    planets.write(b"Pluto");


    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech);

    let planets = fs::read_to_string("planets.txt").unwrap();
    // println!("{planets}");

    for (index, line) in planets.lines().enumerate() {
        println!("line {index} is: {line}");
    }

    let planets = fs::read("planets.txt").unwrap();
    println!("{planets:?}");




    for (index, argument) in env::args().enumerate() {
        println!("argument {index} is {argument}");
    }

    if env::args().len() <= 2 {
        println!("You need more args.");
    } else {
        let arg1 = env::args().nth(1).unwrap();
        let arg2 = env::args().nth(2).unwrap();
        println!("one: {arg1} and two: {arg2}");
    }
    */

}
