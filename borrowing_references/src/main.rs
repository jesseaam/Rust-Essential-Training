fn main() {
    let test1 = String::from("jesse boy");
    let test3 = String::from("jesse boy how are you?");
    let test5 = String::from("jesse \u{20BF}  ");
    // let (a, b) = pop_first(&test3);
    // let (c, d) = pop_first(&b);
    // println!("a is: '{a}'\nand b is: '{b}'");
    // println!("c is: '{c}'\nand d is: '{d}'");
    let n = 3;
    let second = pop_nth(&test3, n);
    println!("The whole string is: '{test3}'");
    println!("The second word is: '{second}'");




    /*
    let test1 = " \tWe need more space.\n";
    // println!("{test1}");
    let new_string = trim_spaces(test1);
    println!("{new_string}");
    assert_eq!(trim_spaces(test1), "We need more space.");
    trim_spaces(test1);

    let test2 = String::from("    There's space in the front.");
    assert_eq!(trim_spaces(&test2), "There's space in the front.");

    let test3 = String::from("There's space to the rear.   ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = String::from("   We're surrounded by space!   ");
    assert_eq!(trim_spaces(&test4), "We're surrounded by space!");

    let test5 = "      ";
    assert_eq!(trim_spaces(&test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(&test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(&test7), "🚀");

    println!("Tests passed!");
     */
}

fn trim_spaces(s: &str) -> &str {
    // let length = bytes.len();
    let mut start: usize = 0;
    let mut end: usize = 0;

    // loop removes leading spaces
    for (index, character) in s.chars().enumerate() {
        if character != '\t' && character != ' ' {
            start = index;
            break
        } 
    }

    // loop removes trailing spaces
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' && character != '\n' {
            end = s.len() - index;
            break
        }     
    }
    &s[start..end] 
}


    /*
    // slice an array
    let planets = [1, 2, 3, 4, 5, 6, 7, 8]; // no Pluto
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_planets = {inner_planets:?}");


    // slices with emoji
    let money = String::from("\u{20BF} is the base layer.");
    let money_slice = &money[0..3]; // valid
    // let money_slice = &money[0..2]; // not at a valid UTF-8 boundary!
    println!("{money}");
    println!("{money_slice}");

    // slices
    let message = String::from("Greetings from Earth!");
    println!("message is {message}");

    // let last_word = &message[15..15+50];
    let last_word = &message[15..];
    println!("last_word is {last_word}");


    // dangling references. The below code won't compile because of the dangling reference
    let rocket_fuel = dangling_ref();
    println!("rocket_fuel is {rocket_fuel}");

    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("rocket_fuel is {rocket_fuel}");
    println!("rocket_fuel length is {length}");

    myfun();
}
// must tell compiler that the reference we are borrowing is also mutable.
fn process_fuel(propellant: &mut String) -> usize {
    println!("processing propellant {propellant}...");
    propellant.push_str(" is highly flammable."); // modifies string, so original should be mutable
    let length = propellant.len();
    length

}

fn myfun(){
    let mut mystring = String::from("jes");
    mystring.push_str("co");
    println!("{mystring}");
}

// to fix this code we should return the variable and not a reference to it.
fn dangling_ref() -> &String {
    let new_fuel = String::from("RP-1");
    &new_fuel
}
*/


fn pop_first(a: &str) -> (&str, &str) {
    let bytes = a.as_bytes();
    let mut start = 0;
    let mut end = 0;
    let mut in_word = false; 
    for (index, &byte) in bytes.iter().enumerate() {
        if byte != b' ' && in_word == false {
            start = index;
            in_word = true;
        }
        if byte == b' ' && in_word == true {
            end = index;
            break;
        }
        end = index; // in case there are no spaces after the word
    }

    if  start == 0 && in_word == false {
        println!("There are no characters in the borrowed string '{a}'");
    }
    return (&a[start..end], &a[end..]);
}

fn pop_nth(a: &str, n: u8) -> &str {

    let mut count: u8 = 1;
    let (b, c) = pop_first(&a);
    loop {
        let (b, c) = pop_first(&c);
        count += 1;
        if count == n {
            return b;
        }
    }
}