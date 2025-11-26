use core::num;
use std::vec;

fn main() {
    // zip 
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![85, 92, 78];

    let combined: Vec<(&str, i32)> = names.iter().zip(scores.iter().cloned()).collect();
 
    println!("Combined names and scores: {:?}", combined);

    // enumerate
    let fruits = vec!["apple", "banana", "cherry"];
    let  indexed_fruit: Vec<(usize, &str)> = fruits.iter().enumerate().collect();
    println!("Indexed fruits: {:?}", indexed_fruit);

    // flat_map
    let nested_numbers = vec![vec![1, 2], vec![3, 4], vec![5]];

    let flattened: Vec<i32> = nested_numbers.iter().flat_map(|inner_vec| inner_vec.iter()).cloned().collect();
    println!("Flattened numbers: {:?}", flattened);

    // take
    let numbers = vec![10, 20, 30, 40, 50];
    let first_three: Vec<i32> = numbers.iter().take(3).cloned().collect();
    println!("First three numbers: {:?}", first_three);

    // skip
    let numbers_skip = vec![10, 20, 30, 40, 50];
    let after_skipping: Vec<i32> = numbers_skip.iter().skip(2).cloned().collect();
    println!("Numbers after skipping first two: {:?}", after_skipping);
}