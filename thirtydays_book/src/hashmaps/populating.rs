use std::collections::HashMap;
fn main() {
    // Example 2 
    let fruit_data = [
        (String::from("apple"), 5),
        (String::from("banana"), 12),
        (String::from("orange"), 8),
        (String::from("grape"), 25),
    ];

    let mut fruits_counts : HashMap<String, i32>  = HashMap::new();

    for (fruit, count) in fruit_data.iter() {
        fruits_counts.insert(fruit.clone(), *count);
    } 

    println!("Fruit count after populating from array: {:?}", fruits_counts);

    // Example 1
    let mut fruit_counts: HashMap<String, i32> = HashMap::new();
    fruit_counts.insert(String::from("apple"), 5);
    fruit_counts.insert(String::from("banna"), 12);
    fruit_counts.insert(String::from("orange"), 8);
    fruit_counts.insert(String::from("grape"), 25);

    let old_apple_count = fruit_counts.insert(String::from("apple"), 7);

    match old_apple_count {
        Some(count) => println!("Previously, there were {} apples", count),
        None => println!("This is the first time we've seen apples"),
    }

    println!("Current fruit counts: {:?}", fruit_counts);
}