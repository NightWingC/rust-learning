fn main() {
    let mut x = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");

    let mut counter = 0;
    println!("The value counter: {counter}");

    counter = counter + 1;
    println!("The value counter: {counter}");

    counter = counter + 1;
    println!("The value counter: {counter}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The number of spaces is: {}", spaces);

    let mut example = 5;
    println!("Initial value: {}", example);

    // example = "Hello"; // This would cause a type error
    // println!("New value: {}", example);
    
    let x = 5;
    println!("The value of x is: {}", x);

    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }

    let mut y = 10;
    println!("the mutable y is: {}", y );

    y = y + 5;
    println!("The updated mutable y is : {}", y);

    // Constants
    const PI: f64 = 3.14159265;
    const MAX_USERS : u32 = 1_000_000;
    const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

    const SECONDS_IN_MINUTE: u32 = 60;
    const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * 60;

    // Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);

    let count = 0;
    println!("Initial count: {}", count);

    let count = count + 1;
    println!("Count after first shadow: {}", count);

    let count = "one";
    println!("Count after second shadow: {}", count); 

}