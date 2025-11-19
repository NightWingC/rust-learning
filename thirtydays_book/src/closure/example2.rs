fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);

    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum); 

    let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    let long_uppercase_words: Vec<_> = words
        .iter()
        .filter(|w| w.len() > 4)
        .map(|w| w.to_uppercase())
        .collect();

    println!("Original words: {:?}", words);
    println!("Long and uppercase words: {:?}", long_uppercase_words);

    let v3 = vec![1, 2, 3, 4, 5,6,7,8,9,10];
    let event_numbers_iterator = v3.iter().filter(|x| **x % 2 == 0);

    println!("Original vector: {:?}", v3);
    println!("Even numbers: {:?}", event_numbers_iterator);

    let v1 = vec![1, 2, 3];
    let iterator = v1.iter();

    let sqared_iterator = iterator.map(|x| x * x);

    let v2: Vec<_> = sqared_iterator.collect();

    println!("Original vector: {:?}", v1);
    println!("Squared vector: {:?}", v2);
}