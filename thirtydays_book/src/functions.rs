fn main() {
    my_first_function();
    greet("Alice");
    greet("Bob");
    add_numbers(10, 20);

    let product = multiply(6, 7);
    println!("The products is : {}", product);
    let difference = subtract(20, 5);
    println!("The difference is: {}", difference);

    let rect_length = 50;
    let rect_width = 30;
    let calculate_area = calculate_rectangle_area(rect_length, rect_width);

    greet_person("Alice", 30);
    greet_person("Bob", 25);

    println!("The area of rectangle is: {}", calculate_area);

    let my_numbers = vec![1,3,5,8,9];
    match find_first_even_number(&my_numbers) {
        Some(even_num) => println!("The first even number is: {}", even_num),
        None => println!("No even numbers found in the list")
    }

    let odd_numbers = vec![1,3,5,7];
    match find_first_even_number(&odd_numbers) {
        Some(even_num) => println!("The first even number is: {}", even_num),
        None => println!("No even numbers found in the list")
    }

    let (q, r) = divide_and_remainder(10.0, 3.0);
    println!("Quotient: {}, Remainder: {}", q, r);

    let (q_zero, r_zero) = divide_and_remainder(10.0, 0.0);
    println!("Quotient (div by zero): {}, Remainder (div by zero): {}", q_zero, r_zero);

}

fn desmostrate_expressions() -> i32 {
    let base = 10;
    let result = if base > 5 {
        base * 2
    } else {
        base + 5
    };
    result
}

// demostrate statements
fn demostrate_statement(){
    let x = 10;
    let y = x * 2;
    let mut z = y  + 5;
    z = z -1;
    println!("The value of z is: {}", z)
}

fn divide_and_remainder(numerator: f64, denominator: f64) -> (f64, f64){
    if denominator == 0.0 {
        return (f64::NAN, f64::NAN)
    }

    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    (quotient, remainder)
}

fn find_first_even_number(numbers:&[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num); // return the first even number 
        }
    }

    None // If no even number is found, return None
}

fn greet_person(name: &str, age: u8){
    println!("Hello, {}! You are {} years old", name, age);
}

fn calculate_rectangle_area(length: u32, width: u32) -> u32{
    let area = length * width; 
    // println!("The area of rectangle is: {}", area);
    return area;
}

fn my_first_function() {
    println!("This is my first function!!")
}

fn greet(name: &str){
    println!("Hello: {}", name);
}

fn add_numbers(x: i32, y: i32){
    let sum  = x + y;
    println!("the sum of {} and {} is : {}", x, y, sum);
}
//  two ways return value
fn multiply(x: i32 , y: i32) -> i32 {
    return  x * y;
}

fn subtract(x: i32 , y: i32) -> i32 {
    x - y // No semicolon here, this is the return value
}

