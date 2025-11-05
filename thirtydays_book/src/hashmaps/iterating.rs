use std::collections::HashMap;
fn main() {
    // mutable value 
    let mut colors3 = HashMap::new();

    colors3.insert(String::from("Blue"), 10);
    colors3.insert(String::from("Yellow"), 50);

    for value in colors3.values_mut() {
        *value *= 2;
    }

    for ( key, value ) in &colors3 {
        println!("{}: {}", key, value)
    }
    // keys 
    let mut colors2 = HashMap::new();

    colors2.insert(String::from("Blue"), 10);
    colors2.insert(String::from("Yellow"), 50);

    for key in colors2.keys() {
        println!("Key: {}", key)
    }

    // values
    for values in colors2.values() {
        println!("Value: {}", values)
    }

    // immutable borrow
    let mut colors = HashMap::new();

    colors.insert(String::from("Blue"), 10);
    colors.insert(String::from("Yellow"), 50);

    for (key, value) in &colors {
        println!("{}: {}", key, value);
    } 

}