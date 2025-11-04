fn main() {

    // consuming iteration 
    let vec5 = vec![10,20,30,40,50];
    println!("Itering with .into_iter():");
    for element in vec5.into_iter(){
        println!(" {}", element);
    }

    // mutable iteration 
    let mut vec4 = vec![10,20,30,40,50];
    println!("Iterating with .iter_mut():");

    for element in vec4.iter_mut(){
        *element *= 2;
    }

    println!("Vector after mutation: {:?}", vec4);


    // immutable iteration
    let vec3 = vec![10,20,30,40,50];
    println!("Iterating with .iter():");

    for element in vec3.iter(){
        println!(" {}", element);
    }

    // unwrap_or()
    let vec2 = vec![10,20,30,40,50];
    let element_or_default = vec2.get(3).unwrap_or(&0);
    println!("Element at index 3 or dafault: {}", element_or_default);

    let element_or_default_out_of_bounds = vec2.get(10).unwrap_or(&0);
    println!("Element at index 10 or dafault: {}", element_or_default_out_of_bounds);

    // if
    let vec = vec![10,20,30,40,50];
    if let Some(element) = vec.get(1) {
        println!("The element at index 1 is: {}", element)
    } else {
        println!("Index 1 is out bounds")
    }

    if let Some(element) = vec.get(10) {
        println!("The element at index 10 is: {}", element)
    } else {
        println!("Index 10 is out bounds")
    }

    // Get method
    let vector = vec![10,20,30,40,50];
    match vector.get(2) {
        Some(element) => println!("The element ar index 2 is: {}", element),
        None => println!("Index 2 is out of bounds")
    }

    match vector.get(5) {
        Some(element) => println!("The element ar index 5 is: {}", element),
        None => println!("Index 5 is out of bounds")
    }
    // Accesing
    let my_vector = vec![10,20,30,40,50];
    let first_element = my_vector[0];

    println!("The first element is: {}", first_element);

    let third_element = my_vector[2];
    println!("The third element is: {}", third_element);
}