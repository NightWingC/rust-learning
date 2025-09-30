use std::io;
fn main() {
    let x = 5;
    println!("the value of x is: {x}");
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value od x in the inner scope is {x}")
    }
    println!("The value of x is: {x}");

    let mut space = "";
    let spaces = space.len();

    let n: u8 = 255;
    let x2 = 2.0;
    let y: f32 = 3.0;

    // Accending array elements


    // Numeric operations 
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // Multiplication
    let product = 4 * 30;
    // division 
    let quotient = 56. / 32.2;
    let truncated = -5 / 3;
    // Remainder
    let remainder = 43 % 5;
    
    // Boolean type 
    let verdader = true;
    let falso: bool = false; // with explicit type annotation

    // The Cahracter Type 
    let c = "z";
    let z : char = 'ℤ';

    // Compuond types
    // The tuple type
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring tuple
    let ( a, s, d) = tuple;
    println!("S: {s}");
    let five_hundred = tuple.0;
    println!("first value tuple: {five_hundred}");

    // The array type
    let arr = [10,20,30];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    // let a: [i32; 5] = [1,2,3,4,5];
    let a: [i32; 5] = [1,2,3,4,5];

    println!("Please enter an array index");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index : usize = index 
        .trim()
        .parse()
        .expect("Index enterd was not a number");

    let elemet = a[index];
    println!("The value of the element at index {index} is: {elemet}");

    another_function();    // x = 6;
    // println!("The value of x is: {x}"); 
}

fn another_function() {
    println!("Another function");
}
