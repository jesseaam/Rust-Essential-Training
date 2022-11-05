fn main() {

    /*
    let mut x: u8 = 254;
    x += 1;
    println!("x = {}", x);
    x = 15;
    println!("x = {}", x);
    */


    // find out what the type of a variable is with an invalid method call on it. handy little trick
    // let x = 10.0;
    // x.what_is_this()

    println!("Learning about floating decimal point numbers.");
    let x = 10.1234567891011121314;
    println!("x = {}", x);
    let x: f32 = 10.1234567891011121314;
    println!("x = {}", x);
    // x = 10.123456789101112 -> 64 bit float
    // x = 10.123457  -> 32 bit float

    println!("\n\nLet's learn about arithmetic operations.");
    let a = 5;
    let b = 6;
    let c = a + b;
    println!("a + b = {}", c);

    // floats
    let a = 5.0;
    let b = 6.0;
    let c = a / b;
    println!("a / b = {:.4}", c); // decimal point precision of 4

    // casting
    let a = 5;
    let b = 6.0;
    let c = (a as f64) * b;
    println!("a * b = {}", c);

    // print statement experimentation
    println!("\nLet's learning about print statement formatting.");
    let x = 10.888889;
    println!("x = {:07.3}", x); //x = 010.889
    println!("a = {0}, b = {1}, a = {0}", a,b );

    println!("Capture identifiers in formated strings! e.g., x = {x}");

    // bitwise operations
    println!("\nLet's learn about bitwise operations.");
    let x = 0b1110_1111u8; // specify u8 to save on mem allocation
    println!("x = {x}");
    println!("As binary: x = {x:8b}");
    let mut x = 0b0010_1111u8;
    println!("x = {x}");
    println!("As binary: x = {x:08b}");
    x = !x;
    println!("NOT operation. x = {x:08b}");

    x = x & 0b1110_0011u8;
    println!("AND 1110_0011 -> {x:08b}");
    let a = x & 0b0100_0000;
    println!("The value at bit 7 is {}", (a > 0) as i8);
    println!("The value at bit 4 is {}", ((x & 0b0000_1000) > 0) as i8);

    x = 0b0011_1100u8;
    println!("x = {x:08b}");
    println!("x OR 0101_0101 -> {:08b}", x | 0b0101_0101);
    println!("x XOR 0101_0101 -> {:08b}", x ^ 0b0101_0101);
    println!("x left shift bit operator, 3 shifts -> {:08b}", x << 3);
    println!("x right shift bit operator, 2 shifts -> {:08b}", x >> 2);



    // booleans
    println!("\nLet's learn some booleans.");
    let a = true;
    let b = false;
    println!("a is {a} and b is {b}");
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^ b);
    println!("NOT a is {}", !a);
    println!("NOT b is {}", !b);

    let c = !(a ^ b) & a;
    println!("NOT (a XOR b) AND a is {c}");

    // short circuiting logical operators
    let d = true || panic!();
    println!("d is {d}");

    println!("\nLet's learn about comparison operations.");
    let a = 5;
    let b = 4;
    println!("a = {a} and b = {b}");
    println!("a > b is {}", a > b);
    println!("a < b is {}", a < b);
    println!("a == b is {}", a == b);
    println!("a != b is {}", a != b);
    println!("a >= b is {}", a >= b);
    println!("a <= b is {}", a <= b);


    println!("\nLet's learn about character literals!");
    let letter = 'a';
    let number = '1';
    let bitcoin = '\u{20BF}';
    let middle_finger = '\u{1F595}';
    let fiat = '\u{1F4B8}';
    println!("letter is {letter}\nnumber is {number}\n{bitcoin} {middle_finger} {fiat}");


    println!("\nChallenge! Find the average of 3 numbers");
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    /* Your code goes here */
    let average = (a as f64 + b + c as f64) / 3.0;
    assert_eq!(average, 45.1);
    println!("Test passed!");








}

