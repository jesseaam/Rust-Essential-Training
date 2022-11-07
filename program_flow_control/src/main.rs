fn main() {
    // loops

    // Challenge: Max, Min, Mean
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    mean = 0.0;
    min = numbers[0];
    max = numbers[0];

    for num in numbers {
        mean += num as f64;
        if num < min {
            min = num;
        } else if num > max {
            max = num;
        }
    }
    mean /= numbers.len() as f64;


    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!")

    /*

    // nested loop
    let mut myarray = [[1, 2, 3, 4],
                       [5, 6, 7, 8],
                       [9, 10, 11, 12]];
    for row in myarray.iter_mut() {
        for num in row.iter_mut() {
            *num += 10; 
            print!{"{num}\t"};
        }
        println!{""};
    }
    // let mut x = 0;

    for j in 0..5 {
        println!("{j}");
    }


    let letters = ['a', 'b', 'c'];
    for (j, &k) in letters.iter().enumerate() {
        if k == 'c' {
            break;
        }
        println!{"letter {k} at index {j}"};
    }


    while x < letters.len() {
        println!{"letter {x} equals {}", letters[x] };
        x += 1;
    }
    println!{"the final value of x is {x}"};

    let result = loop {
        println!{"x is {x}"}
        if x >= 10 {
            break x * 10;
        }
        x += 1;
    };
    println!{"The result of {x} * 10 = {result}"};

    loop {
        x += 1;
        if x > 5 {
            break;
        }
    }
    println!{"x = {x}"};

    loop { // infinite loop you have to manually stop
        x += 1;
        println!{"x = {x}"};
    }
    
    // if else statements
    let make_x_odd = false;
    let x = if make_x_odd {1} else {2};
    println!("x is {x}")
    let x = 5;
    let y = 4;

    if x + 1 != 3 {
        println!{"x = {x}"}
    }

    if x > y {
        println!{"x is greater than y"};
    } else if y > x {
            println!("x is less than y");
    } else {
        println!{"x is equal to y"};
    }
    
    */
}
