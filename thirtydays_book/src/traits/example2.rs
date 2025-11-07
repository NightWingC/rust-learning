use std::fmt::Display;

fn create_printer<T: Display>(message: &'static str) -> impl Fn(&T) + '_ {
    move |value: &T| {
        println!("{}: {}", message, value);
    }
}

fn create_summer(numbers: Vec<i32>) -> impl FnMut() -> i32 { 
    move || { 
        numbers.iter().sum() 
    } 
}


fn main() {
    let display_int = create_printer("Integer value");
    display_int(&10);
    display_int(&25);

    let display_string = create_printer("String value");
    display_string(&"Hello");

    let mut summer_five = create_summer(vec![1,2,3,4,5]);
    println!("Sum: {}", summer_five());

    let mut summer_ten = create_summer(vec![10,20,30,40,50]);
    println!("Sum: {}", summer_ten());
}