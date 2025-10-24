fn main(){
    // Definig a tuple with different data types
    let tup: (i32, f64, u8) = (-500, 6.4, 1);
    // Accssing tuple elements using dot notation and their index
    let first_element = tup.0;
    let last_element = tup.2;

    println!("The first element of tuple is: {}", first_element);
    println!("The last element of tuple is: {}", last_element);

    // tuples can cointain any type, including other tuples (nested tuples)
    let nested_tuple = (5,(10,20), "a");
    let nested_second_inner = nested_tuple.1.0;

    println!("Nested tuple second inner element: {}", nested_second_inner);

    // destructuring a tuple into separate variables
    let  (x,y,z) = tup;
    println!("Destructured tuple elements: x={}, y={}, z={}", x,y,z);

    // Destructuring with nested tuples
    let ( a, (b,c), d) = nested_tuple;
    println!("Destructured nested tuple elements: a={}, b={}, c={}, d={}", a,b,c,d);


    let ( result_string, result_number) = process_data();
    println!("Function returned: \"{}\", {}", result_string, result_number);
    
    // tuples can be empty, represented by ()
    let unit_tuple: () = ();
    println!("The unit tuple: {:?}", unit_tuple);

    // Tuples do not need to have the same type, but they do have fixes sizes
    let mixed_tuple = ("Rust", 2024, true);
    println!("A mixed tuple: {:?}", mixed_tuple);

    // You can ignore elements you do not neeed using the underscore
    let ( name, _ , active ) = ("Alice", 30, false);
    println!("Name: {}, Active: {}", name, active);

    // The tuple are immutable by default, if you want to change a tuple you must declare is as mutable
    let mut mutable_tuple = (10, 20);
    println!("Mutable tuple: {:?}", mutable_tuple);

    mutable_tuple.0 = 30;
    println!("Mutable tuple after change: {:?}", mutable_tuple);

    // however, even with a mutable tuple, you cannot change the type of an element

    // mutable_tuple.0 = "hello"; // This would result in a compile time error.

    // The tuples are compared element by element
    let tuple_a = (1,2);
    let tuple_b = (1,3);
    let tuple_c = (2,1);

    println!("tuple a == tuple b: {}", tuple_a == tuple_b);
    println!("tuple a < tuple b: {}", tuple_a < tuple_b);
    println!("tuple a < tuple c: {}", tuple_a < tuple_c);


}

fn process_data() -> (String, i32) {
    let data_string = String::from("Processed data");
    let data_number = 12345;
    (data_string, data_number)
}