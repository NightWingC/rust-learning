fn main(){

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("S1 = {s1}, S2 = {s2}");

    let x = 5;
    let y = x;

    println!("x: {x}, y: {y}");

    let s = String::from("Hello"); // s comes into scope 
    takes_ownership(s); // s's values move into the function...

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function 
}

fn takes_ownership(some_string: String){
    println!("{some_string}")
}

fn makes_copy(some_integer: i32){
    println!("{some_integer}")
}

// -------------- //
fn exercise2(){
    let s1 = String::from("Hello");
    let (s2, len) = calculate_lenght(s1);

    println!("The length of {s2} is {len}");
}

fn calculate_lenght(s: String) -> (String, usize){
    let lenght = s.len(); // len() returns the length of a string
    ( s, lenght )
}

// ---------- //

fn exercise3(){
    let s1 = String::from("Hello");
    let len = calculate_lenght(&s1);

    println!("The length of {s1} is {len}");
}

fn calculate_lenght(s: &String) -> usize {
    s.len() 
}