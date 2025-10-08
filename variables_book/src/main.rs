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