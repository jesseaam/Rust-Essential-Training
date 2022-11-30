// Define a struct to represent a Rectangle
// Fields: width & height (f64, f64)
// methods: get_area(); scale();
// function: new(w, h) to create/return new rectangle

// BONUS: define a struct to represent a Circle with same methods and function

struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, scalar: f64)  {
        self.width *= scalar;
        self.height *= scalar;
    }

    fn new(w: f64, h: f64) -> Rectangle {
        Rectangle {
            width: w,
            height: h
        }
    }
}

struct Circle {
    radius: f64
}

impl Circle {
    fn scale(&mut self, scalar: f64) {
        self.radius *= scalar;
    }

    fn get_area(&self) -> f64 {
        3.14 * self.radius.powf(2.0)
    }

    fn new(radius: f64) -> Circle {
        Circle {
            radius
        }
    }
}

fn main() {

    // Unit tests
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);

    let mut circ = Circle::new(1.0);
    assert_eq!(circ.get_area(), 3.14);
    circ.scale(2.0);
    assert_eq!(circ.get_area(), 12.56);

    println!("Tests passed!");
}




/*
// Tuple Structs
struct Color(u8, u8, u8); // RGB 
struct Point(u8, u8, u8); // XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
    let red = Color (255, 0, 0);
    println!("First value is {}", red.0);

    let coord = Point(4, 5, 6);
    let y = get_y(coord);
    println!("y is {y}");
}





// Struct associated Functions
#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0 
        }
    }
}

fn main() {
    let mut vehicle = Shuttle::new("Endeavor");
    let mut vehicle2 = Shuttle::new("Discovery");
    println!("{vehicle:?}");
    println!("{vehicle2:?}");
}





// Struct Methods
#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavor"),
        crew_size: 6,
        propellant: 0.0
    };
    
    let vehicle_name = vehicle.get_name();
    println!("{vehicle_name}");

    println!("gallons of propellant: {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("gallons of propellant: {}", vehicle.propellant);

    // let vehicle_name = vehicle.get_name();
    // println!("{vehicle_name}");
}





// Struct
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

    println!("Shuttle name: {}", vehicle.name);
    println!("Crew size: {}", vehicle.crew_size);
    println!("Gallons of propellant: {}", vehicle.propellant);
}
*/