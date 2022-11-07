fn main() {


    // transfer ownership
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket_fuel is {rocket_fuel}");

    fn process_fuel(propellant: String) -> String {
        println!("processing propellant: {propellant}");
        let new_fuel = String::from("LNG");
        return new_fuel;
    }
    

    /*
    let rocket_fuel = 1;
    process_fuel(rocket_fuel);
    println!("rocket_fuel is {rocket_fuel}");

    fn process_fuel(mut propellant: i32) {
        propellant += 1;
        println!("processing propellant: {propellant}");
    }

    // example of copy
    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        outer_planet = inner_planet;
        println!("{inner_planet}");
        inner_planet += 1;
    }
    println!("{outer_planet}");

    // example of a clone
    let outer_planet: String;
    {
       let mut inner_planet = String::from("Mercury");
       outer_planet = inner_planet.clone();
       inner_planet.clear();
       println!("{inner_planet} -> inner");
    }
    println!("{outer_planet} -> outer");

    // example of a Move
    let outer_planet: String;
    {
       let inner_planet = String::from("Mercury");
       println!("{inner_planet}");
       outer_planet = inner_planet;
    }
    println!("{outer_planet}");


    // strings -> on the heap
    let mut greeting  = String::from("Hello");
    greeting.push_str(", world!");
    println!("{greeting}");


    // shadowing example
    let x = 5;
    if true {
        let x = 6;
        println!("x is {x}");
    }
    println!("x is {x}");
    
    */
}
