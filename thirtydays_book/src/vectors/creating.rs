fn main() {
    // truncate
    let mut v4 = vec![1,2,3,4,5];
    v4.truncate(3);

    println!("Vector after truncate: {:?}", v4);

    // Remove element 
    let mut v3 = vec![1,10,2,3,4];
    let removed_element = v3.remove(1);
    println!("Removed element: {}", removed_element);
    println!("Vector elements: {:?}", v3);

    // Insert in the vector
    let mut v2 = vec![1,2,3,4];
    v2.insert(1,10);

    println!("Vector after insert: {:?}", v2);


    let mut v = vec![1, 2, 3]; 
    let last_element = v.pop(); // last_element will be Some(3) 
    println!("Popped element: {:?}", last_element); // Output: Popped element: Some(3) 
    println!("Vector after pop: {:?}", v); 
    
    let second_last_element = v.pop(); // second_last_element will be Some(2) 
    println!("Popped element: {:?}", second_last_element); // Output: Popped element: Some(2) 
    println!("Vector after pop: {:?}", v);        // Output: Vector after pop: [1] 

    let only_element = v.pop(); // only_element will be Some(1) 
    println!("Popped element: {:?}", only_element); // Output: Popped element: Some(1) 
    println!("Vector after pop: {:?}", v);        // Output: Vector after pop: [] 

    let empty_pop = v.pop(); // empty_pop will be None 
    println!("Popped element: {:?}", empty_pop);   // Output: Popped element: None
    println!("Vector after pop: {:?}", v);         // Output: Vector after pop: []



    let v2 = vec![100,200,300];
    let first = v2.get(0);
    let third = v2.get(2);
    let sixth = v2.get(5);

    // println!("v2.get(0): {}", first);
    // println!("v2.get(2): {}", third);
    // println!("v2.get(5): {}", sixth);

    if let Some(element) = v2.get(1) {
        println!("The second element is: {}", element)
    } else {
        println!("Index 1 is out of bonus.")
    }

    if let Some(element) = v2.get(10) {
        println!("The eleventh element is: {}", element)
    } else {
        println!("Index 10 is out of bonus.")
    }




    let v = vec![10,20,30,40,50];
    let first_element = v[0];
    let third_element = v[2];

    println!("First element: {}", first_element);
    println!("Third element: {}", third_element);

    let v_turbofish = Vec::<f64>::new();
    println!("v_turbofish: {:?}", v_turbofish);

    let mut v_explicit_type: Vec<String> = Vec::new();
    v_explicit_type.push(String::from("Hello"));
    v_explicit_type.push(String::from("World"));

    println!("v_explicit_type: {:?}", v_explicit_type);

    let v_repeated = vec![0; 5];
    println!("v_repeated: {:?}", v_repeated);

    let vec_macro = vec![1,2,3,4,5];
    println!("v_macro: {:?}", vec_macro);

    let mut v_new: Vec<i32> = Vec::new();
    v_new.push(5);
    v_new.push(10);
    v_new.push(15);

    println!("v_new after pushes: {:?}", v_new);

}