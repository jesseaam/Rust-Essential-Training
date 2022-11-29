#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };
    /*
    println!("Shuttle name: {}", vehicle.name);
    println!("Crew size: {}", vehicle.crew_size);
    println!("Gallons of propellant: {}", vehicle.propellant);
    */
    let mut vehicle2 = Shuttle {
        // name: String::from("Discovery"),
        ..vehicle.clone()
    };

    vehicle.crew_size = 6;

    println!("{vehicle:?}");
    println!("{vehicle2:?}");
}
