use std::ops::Add;

fn largest_i32(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn largest_f64(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

fn largest<T: PartialOrd>(a: T, b:T ) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn swap<T, U>(a: &mut T, b: &mut U) {

}

fn swap_same_type<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

fn sum_slice<T: Add<Output = T> + Copy>(slice: &[T] ) -> T {
    let mut total = slice[0];
    for &item in slice.iter().skip(1) {
        total = total + item;
    }
    total
}

fn main() {
    let numbers = [1,2,3,4,5];
    let sum_numbers = sum_slice(&numbers);
    println!("Sum of numbers: {}", sum_numbers);

    let floats = [1.1,2.2,3.3,4.4];
    let sum_floats = sum_slice(&floats);
    println!("Sum of floats: {}", sum_floats);

    println!("-------------------------");



    let mut x = 5;
    let mut y = 10;
    println!("Before swap: x = {}, y = {}", x, y);

    swap_same_type(&mut x, &mut y);
    println!("After swap: x = {}, y = {}", x, y);

    let mut s1 = String::from("hello");
    let mut s2 = String::from("world");

    println!("Before swap: s1 = {}, s2 = {}", s1, s2);
    swap_same_type(&mut s1, &mut s2);

    println!("After swap: s1 = {}, s2 = {}", s1, s2);

    let num1 = 10;
    let num2 = 20;
    let result_int = largest(num1, num2);
    println!("The largest integer is: {}", result_int);

    let float1 = 3.14;
    let float2 = 2.71;
    let result_float = largest(float1, float2);
    println!("The largest float is: {}", result_float);

    let char1 = 'a';
    let char2 = 'b';
    let result_char = largest(char1, char2);
    println!("The largest char is: {}", result_char);

}

