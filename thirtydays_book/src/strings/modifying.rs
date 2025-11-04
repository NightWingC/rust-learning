use std::fmt::format;

fn main() {
    // format!
    let ex1 = String::from("tic");
    let ex2 = String::from("tac");
    let ex3 = String::from("toe");
    
    let game = format!("{} - {} - {}", ex1, ex2, ex3);
    println!("Formatted string \"{}\"", game);
    println!("Original strings: \"{}\", \"{}\", \"{}\"", ex1, ex2, ex3);

    let e1 = String::from("Hello");
    let e2 = "- world";
    let e3 = e1 + e2;
    println!("Concatenated string: \"{}\"", e3);
    // println!("{}", s1); // This line would cause a compile-time error because s1 has been moved.

    // .pust_strc
    let mut s3 = String::from("foo");
    let suffix = "bar";

    s3.push_str(suffix);
    println!("String after push_str: \"{}\"", s3);
    println!("The original suffix is still valid: \"{}\"", suffix);

    let data = "initial contents";
    let s1 = data.to_string();
    println!("String from literal: \"{}\"", s1);

    let number = 123;
    let s_from_number = number.to_string();
    println!("String from number \"{}\"", s_from_number);

    let mut s = String::new();
    println!("An empty string: \"{}\"", s);
}