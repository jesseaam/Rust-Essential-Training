// Challenge: Represent a location
// Define an enum named Location to represent a location
// 1. Unknowned, 2. Anonymous, 3. Known - with lat and long stored as float values
// implement a display() method to print location information
// output should differ based on the variant

enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Unknown"),
            Location::Anonymous => println!("Anon"),
            Location::Known(lat, long) => println!("{lat} {long}")
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();

}

/*
// if-let syntax

fn main() {
    let number = Some(13);

    // match number {
    //     Some(13) => println!("thirteen"),
    //     _ => ()
    // }
    if let Some(13) = number {
        println!("thirteen")
    }
}




// Mathing Option<T>

fn main() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(2);
    let number = match number {
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {number:?}");

}





// Option<T> enum

fn main() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    let number = number.unwrap_or(&0) + 1;
    println!("number is {number:?}");

}



// enum methods

#[derive(Debug)]
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
    Triangle(f64, f64, f64) // sides a, b, c
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c
        }
    }
}

fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {my_shape:?}");
    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {perimeter:?}");
}





// Match with default placeholders

fn main() {
    let my_number = 4u8;

    let results = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("{my_number} did not match");
            "something else"
        }
    };
    println!("{results}");
}




// match operator
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}



fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {my_shape:?}");

    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {r}"),
        Shape::Rectangle(w, h) => println!("{w} x {h} Rectangle", w, h),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {a}, {b}, {c}")
    }
}

*/