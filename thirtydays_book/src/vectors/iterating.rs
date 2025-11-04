fn main(){
    let v8 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    println!("Filtering even numbers and then squaring them:");

    let processed_numbers: Vec<i32> = v8.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();

    println!("Processed numbers: {:?}", processed_numbers);

    let v7 = vec![1, 2, 3, 4, 5,];
    println!("Mapping elements (squaring):");
    for squared_number in v7.iter().map(|&x| x * x){
        println!(" {}", squared_number);
    }

    let v6 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Filtering even numbers:");
    for number in v6.iter().filter(|&&x| x % 2 == 0) {
        println!("{}", number);
    }

    let v5 = vec![String::from("hello"), String::from("world")];
    println!("Using into_iter():");
    
    let mut iter_owned = v5.into_iter();
    println!("First owned element: {:?}", iter_owned.next());
    println!("Second owned element: {:?}", iter_owned.next());
    println!("Next element: {:?}", iter_owned.next());


    
    let mut v4 = vec![10,20,30];
    println!("Using .iter_mut");

    let mut iter_mut = v4.iter_mut();
    if let Some(first_mut_iter) = iter_mut.next() {
        *first_mut_iter = 100;
        println!("First element mutated to: {}", first_mut_iter);
    }  

    for element_mut_ref in v4.iter_mut() {
        *element_mut_ref *= 2;
        println!("Doubled element: {}", element_mut_ref)
    }

    let v3 = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    println!("Using .iter():"); 
    let mut iter = v3.iter(); // Manually calling next() 
    println!("First element: {:?}", iter.next()); // Some(&1.1) 
    println!("Second element: {:?}", iter.next()); // Some(&2.2) 
    // Using iter() in a for loop is equivalent to iterating over &v 
    for element_ref in v3.iter() { 
        println!("Element via .iter(): {}", element_ref); 
    }

    let mut v = vec![1, 2, 3, 4, 5]; 
    println!("Iterating and mutating elements:"); 
    for element in &mut v { 
        *element += 5; // Dereference to modify the value 
        println!("Modified value: {}", element); 
    } 
    println!("Vector after mutation: {:?}", v);
 

    let v2 = vec![String::from("apple"), String::from("banana"), String::from("cherry")];
    println!("Iterating over Strings:");

    for fruit in &v2 {
        println!("Fruit: {}", fruit);
    }


    let v = vec![10, 20, 30, 40, 50];

    println!("Iterating with immutable references:");
    for element in &v {
        println!("The value is: {}", element);
    }
}