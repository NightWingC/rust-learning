use std::fs::File;
use std::io::ErrorKind;
struct Point {
    x: i32,
    y: i32
}
fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 3 == 0{
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4 , 3 or 2");
    }

    let condition = true;
    let number_condition = if condition {
        5
    } else {
        6
    };

    println!("The value of number condition is: {}", number_condition); 

    // Example domostrating type compatibility for assignment
    let number_a = 7;
    let message = if number_a > 10 {
        "Greater than 10"
    } else if number_a < 10 && number_a > 5 {
        "Between 5 and 10 "
    } else {
        "5 or less"
    };

    println!("The message is: {}", message);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter  == 10 {
            break counter * 2;
        }
    };

    println!("the result is {}", result);

    let mut count = 0;
    loop {
        println!("Looping...");
        count += 1;
        if count >= 5 {
            break;
        }
    }

    let mut i = 0;
    loop {
        println!("i is: {}", i);
        i += 1;
        if i == 3 {
            continue;
        }

        if i > 5 {
            break;
        }
    }

    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    println!("{:?}", a);

    // Iterating over a range
    for number in 1..4 {
        print!("{}!", number)
    }

    for number in 1..=4 {
        print!("{}!", number)
    }

    // iterating array in reverse
    for number in a.iter().rev() {
        println!("number: {}", number)
    }


    // Match
    let x =5;
    match x {
        1 => println!("One!"),
        2 => println!("One!"),
        3 => println!("One!"),
        4 => println!("One!"),
        5 => println!("One!"),
        _=> println!("Other"),     
    };

    let tuple: (i32, char, u8) = (5, 'a', 1);
    match tuple {
        (0, _, _) => println!("First value is 0"),
        (1,'a',_) => println!("First is 1, secont is a"),
        (_,'b',_) => println!("Second value is b"),
        (_,_,_) => println!("Default case"),
    }

    let p = Point { x: 0, y: 10 };
    match p {
        Point { x: 0, y} => println!("On the y-axis at {}", y),
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point {x,y} => println!("Point ({}, {})", x,y),
    }

    let some_value = Some(3);
    let another_value = Some(5);
    let absent_value: Option<i32> = None;

    match some_value {
        Some(3) => println!("three"),
        Some(5) => println!("five"),
        _=> println!("Other"),
    }

    match absent_value {
        Some(x) => println!("Got value"),
        None => println!("No value"),
    }

    let file = File::open("hello.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    }; 


    let coin = "penny";
    let count = match coin {
        "penny" => 1,
        "nickel" => 5,
        "dime" => 10,
        "quarter" => 25,
        _ => 0
    };

    println!("Coin value is: {}", count);
}