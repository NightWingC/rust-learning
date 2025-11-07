use std::fmt::Display;
use std::fmt::Debug;
use std::cmp::PartialOrd;

fn print_item<T: Display>(item: &T){
    println!("{}", item)
}

fn create_adder(base: i32) -> impl Fn(i32) -> i32 {
    move |x | x + base
}

fn print_item_and_debug<T>(item: &T)
    where T: Display + Debug,
    { 
        println!("Display: {}", item); 
        println!("Debug: {:?}", item); 
    }

struct Pair<T,U>
where 
    T: PartialOrd + Debug,
    U: PartialOrd + Debug
    {
        first: T,
        second: U,
    }

impl<T, U> Pair<T, U>
where 
    T: PartialOrd + Debug,
    U: PartialOrd + Debug, {
        fn new(first: T, second: U) -> Self {
            Pair { first, second }
        }

        fn is_first_greater(&self) -> bool {
            self.first > self.second
        }
    }

fn main() {

    let p1 = Pair::new(5, 10.5);
    println!("Pair 1: {:?}", p1);
    

    let data = (1, "hello", 3.14);
    print_item_and_debug(&data);

    println!("----------------------");


    let add_five = create_adder(5);
    println!("{}", add_five(10));

    println!("----------------------");

    let integer = 5;
    let float = 10.5;
    let message = "Hello, rust!";

    print_item(&integer);
    print_item(&float);
    print_item(&message);  

    let string_val_ref: &String = &String::from("Another string");
    print_item(string_val_ref);   
}