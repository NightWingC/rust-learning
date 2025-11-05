use std::collections::HashMap;
fn main() {
    // Remove
    let mut score5 = HashMap::new();
    score5.insert(String::from("Alan"), 100);
    if score5.contains_key("Alan") {
        score5.remove("Alan");
        score5.insert(String::from("Alan"), 115);
    }

    println!("Alan's score after remove and insert: {:?}", score5.get("Alan"));

    // modify
    let mut scores4 = HashMap::new();
    scores4.insert(String::from("Rosy"), 100);
    scores4.entry(String::from("Rosy"))
    .and_modify(|score| *score += 5)
    .or_insert(50);

    scores4.entry(String::from("Rob"))
    .and_modify(|score| *score += 5)
    .or_insert(50);

    println!("Scores: {:?}", scores4);

    // or_insert_with
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Pep"), 100);
    let ben_score = scores3.entry(String::from("Ben")).or_insert_with(|| {
        println!("Calculating default score for Ben...");
        0
    });

    println!("Ben's score using or_insert_with: {}", ben_score);

    // insert_or
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Alice"), 100);

    let aslice_score = scores2.entry(String::from("Alice")).or_insert(0);
    *aslice_score += 10;
    println!("Alice's score after or_insert and increment: {}", aslice_score);

    let bobs_score = scores2.entry(String::from("Bob")).or_insert(0);
    println!("Bob's score after or_insert: {}", bobs_score);

    // 
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 95);


    let alice_score =  scores.get("Alice");
    match  alice_score {
        Some(score) => println!("Alice's score is : {}", score),
        None => println!("Alice's score not dound"),
    }

    let charlie_score = scores.get("Cahrlie");

    match charlie_score {
        Some(score ) => println!("Charlie's score is: {}", score),
        None => println!("Charie's score not found")
    }

    if let Some(bob_score) = scores.get_mut("Bob") {
        *bob_score += 5;
        println!("Bob's updated score is: {}", bob_score);
    } else {
        println!("Bob's score not found, cannot opdated.")
    }

    if let Some(charlies_score) = scores.get_mut("Charlie") {
        *charlies_score += 5;
        println!("Charlie's updated score is: {}", charlies_score);
    } else {
        println!("Charlie's score not found, cannot opdated.")
    }




}