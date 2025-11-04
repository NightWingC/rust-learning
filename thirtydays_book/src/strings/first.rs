fn main(){

    // To append another String without consuming it, you can use format! 
    let s1 = String::from("Hello"); 
    let s2 = String::from(", "); 
    let s3 = String::from("world!"); 
    let combined = format!("{}{}{}", s1, s2, s3); 
    println!("Combined string using format!: {}", combined); 
    println!("s1 is still valid: {}", s1);

    // Concatenated
    let part1 = String::from("This ");
    let part2 = "is ";
    let part3 = "concatenated.";
    let final_string = part1 + part2 + part3;
    println!("Final concatenated string: {}", final_string);

    // to_String
    let hello_slice: &str = "Hello";
    let world_slice: &str = "World";

    let mut greeting_string: String = String::from(hello_slice);

    println!("Initial string: {}", greeting_string);
    greeting_string.push_str(world_slice);

    println!("Appended string: {}", greeting_string);

    let my_tring = String::from("This is my string");
    let string_slice: &str = &my_tring;
    println!("String slice: {}", string_slice);

    let another_slice: &str = my_tring.as_str();
    println!("Another string slice: {}", another_slice);

    print_string_slice(string_slice);
    print_string_slice(&string_slice);
    print_string_slice("Hello from literal!");

}

fn print_string_slice(s: &str) {
    println!("Received slice: {}", s);
}