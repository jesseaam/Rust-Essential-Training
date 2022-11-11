fn main() {
    let test1 = "We need more space.";
    trim_spaces(test1);
    assert_eq!(trim_spaces(test1), "We need more space.");

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
}

fn trim_spaces(s: &str) -> &str {
    // let length = bytes.len();
    let mut start: usize = 0;
    let mut end: usize = 0;

    // loop removes leading spaces
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }     
    }
    
    // loop removes trailing spaces
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
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