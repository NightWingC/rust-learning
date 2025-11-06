
fn get_first_element(data: &[i32]) -> i32 {
    if data.is_empty(){
        panic!("Cannot get the first element of an empty slice");
    }
    data[0]
}

fn process_positive_number(num: i32){
    if num < 0 {
        panic!("Input muts be a positive number, but received: {}", num);
    }

    println!("Processing positive number: {}", num);
}
fn main() {
    // example 2
    process_positive_number(5);
    // process_positive_number(-5);

    // example 1
    let numbers = vec![10,20,30];
    let first = get_first_element(&numbers);
    println!("The first element is: {}", first);

    let empty_numbers : Vec<i32> = Vec::new();
    // The following call will trigger a panic. 
    let first_of_empty = get_first_element(&empty_numbers); 
    println!("The first element of empty is: {}", first_of_empty); // This line will not be reached.

}