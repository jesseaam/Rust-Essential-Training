fn main() {
    // hello_world();
    // let a = 8;
    // let b = 10;
    // add_nums(a, b);
    // let a = square_num(13);
    // println!{"{} squared is {}.", a.0, a.1};
    // println!("{a:?}");
    let celcius_temp = 23.0;
    let fahrenheit_temp = celcius_to_fahrenheit(celcius_temp);

    assert_eq!(fahrenheit_temp, 73.4 as f64);
    println!("Test passed!");
}

fn celcius_to_fahrenheit(celcius_temp: f64) -> f64 {
    celcius_temp * 1.8 + 32.0
}


// fn hello_world() {
//     println!("Hello, world!");
// }

// fn add_nums(a: u8, b: u8) {
//     let c = a + b;
//     println!("a + b = {c}")
// }

// fn square_num(a: i32) -> (i32, i32) {
//     print!{"Squaring {a}... "}
//     return (a, a * a);
// }
