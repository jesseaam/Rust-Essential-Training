// Challenge
// Write a function to add two numbers stored within
// Box<T> objects
// function name: sum_boxes
// Input: two Box<T> objects, where T is a numeric type
// Output: Box<T> containing the sum of the two input values

fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    Box::new(*a + *b)
}

fn compare_boxes<T: std::cmp::PartialOrd>(a: Box<T>, b: Box<T>) {
    if *a > *b {
        println!("a is greater than b");
    } else if *a < *b {
        println!("a is less than b");
    } else {
        println!("a is equal to b");
    }
}

fn main() {

    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);

    let one = Box::new(1);
    let two = Box::new(1);
    compare_boxes(one, two);
    let one = Box::new(1);
    let two = Box::new(2);
    compare_boxes(one, two);
    let one = Box::new(2);
    let two = Box::new(1);
    compare_boxes(one, two);

    println!("\nTests passed!");
}

/*

// Box data type

use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn main() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0
    };
    println!("vehicle size on stack: {} Bytes", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!("boxed_vehicle size on stack: {} Bytes", mem::size_of_val(&boxed_vehicle));
    println!("boxed_vehicle size on heap: {} Bytes", mem::size_of_val(&*boxed_vehicle));

    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!("unboxed_vehicle size on stack: {} Bytes", mem::size_of_val(&unboxed_vehicle));
}



// Generic function definitions
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("The biggest is {}", get_biggest(1, 2));
}



// Generic method definitions

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U 
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }

}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * &self.width + 
        2 * &self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 2,
        height: 4.0
    };

    let rect2 = Rectangle {
        width: 2u8,
        height: 6u8
    };

    println!("{rect:?}");
    println!("rect width is: {:?}", rect.get_width());
    println!("rect2 perimeter is: {:?}", rect2.get_perimeter());
}



// Generic struct definitions

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U 
}

fn main() {
    let rect = Rectangle {
        width: 2,
        height: 4.0
    };

    println!("{rect:?}");
}

*/