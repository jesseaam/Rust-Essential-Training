fn main() {
    // my 1st array in rust
    let letters = ['a', 'b', 'c'];
    println!("Second letter is: {}", letters[1]);

    // let mut numbers = [1, 2, 3, 4];
    // println!("{}", numbers[1]);
    // numbers[0] = 9;
    // println!("{}", numbers[0]);

    let numbers: [i32; 8];
    numbers = [0;8];
    let index: usize = numbers.len();
    println!("{}", numbers[index-8]);

    // let parking_lot: [[i32; 4]; 2];
    // parking_lot = [[1,2,3,4],
    //                [5,6,7,8]];
    // let index1: usize = 1;
    // let index2: usize = 2;
    // println!("At row 0 column 1: {}", parking_lot[index1][index2])

    // let garage: [[[i32; 100]; 5]; 10]; // 100 rows, 5 cols, 10 story
    // let zero_garage = [[[0; 100]; 5]; 10]; // initialize all zeros

    println!("\nLet's learn about tuples.");

    let glovebox = ('\u{20BF}', '\u{1F4B5}', 5, '\u{1F527}');
    let (bitcoin, dollar, five, wrench) = glovebox;
    println!("{} {} {} attack for your {}", five, dollar, wrench, bitcoin);

}

