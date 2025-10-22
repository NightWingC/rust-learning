fn main() {

    // integer 8bits = -128 to 127 
    let a :i8 = -100;
    let b: i8 = 127;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);

    // integer 16bits -32,768 to 32,767
    let temperature: i32 = -20_000;
    let user_counts: i32 = 30_000; 

    println!("The current temperature is: {}", temperature);
    println!("There are {} active users", user_counts);

    // integer 32bits = -2,147,483,648 to 2,147,483,647.
    // integer 64bits = -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807.
    // integer 128bit = -17,014,118,346,046,923,173,168,730,371,588,410,5728 to 17,014,118,346,046,923,173,168,730,371,588,410,5727.

    let large_number: i64 = -90000000000000;
    let transaction_id: i64 = 8500000000000000;

    println!("{large_number}");
    println!("Transaction Id: {}", transaction_id);

    // isizes 
    let count: isize = -5;
    println!("Negative count {}", count);

    // usize
    let numbers = vec![10,20,30];
    let index: usize = 1;
    println!("The element at index {} is: {}", index, numbers[index] );

    // u8 = 255
    // u16 = 0 to 65,535
    // u32 = 0 to 4,294,967,295
    // u64 = 0 to 18,446,744,073,709,551,615
    // u128 = 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455

    // Declaring a variable with type annotation
    let pi_f32_annotated: f32 = 3.14159;
    println!("The value of pi is a: {}", pi_f32_annotated);

    // Rust can often infer the type
    let gravity = 9.81; // By default, inferred as f64
    let gravity_f32: f32 = 9.81f32; // Explicitly specifyting f32

    println!("The valuo of gravity (f32 explicit ): {}", gravity_f32); 

    // Performing arithmetic operations with f32
    let radius: f32 = 5.0;
    let area_f32 = std::f32::consts::PI * radius * radius;

    println!("The area of a circle with radius {}", area_f32); 

    // comparing f32 values can bi tricky due to precision issues
    let x: f32 = 0.1 + 0.2;
    let y: f32 = 0.3;

    // Direct comparison might fail due to tiny precision differences
    if x == y {
        println!("x and y are equal (f32)");
    } else {
        println!("x ({}) and y ({}) are no equal (f32)", x,y);

        // a more robust comparision involves checking if the difference is within an epsilon

        let epsilon = 1e-6; // A small tolerance
        if(x - y).abs() < epsilon {
            println!("x and y are approximately equal (f32)");
        }
    }

    // declaring a variable with type annotation
    let pi_f64_annotated: f64 = 3.141592653589793;

    println!("The value of pi (f64 annotated); {}", pi_f64_annotated);


}