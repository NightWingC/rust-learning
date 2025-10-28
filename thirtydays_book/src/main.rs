enum WebEvent {
    PageLoad,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

enum Shape {
    Circle(f64, f64, f64),
    Rectangle(i32, i32, i32, i32),
    Point,
}

fn main() {

    let number_option = Some(5);
    let squared_option = number_option.map(|num| num * num);

    println!("Squared option: {:?}", squared_option);

    let none_option: Option<i32> = None;
    let mapped_none_option = none_option.map(|num| num * num);
    println!("Mapped none option: {:?}", mapped_none_option);



    let config_setting: Option<&str> = None;

    // let database_url = config_setting.expect("Failed to get database URL from configuration.");
    // println!("Database URL: {}", database_url);

    let some_number = Some(5);
    let unwrapped_number = some_number.unwrap();
    println!("Unwrapped number: {}", unwrapped_number);

    let config_value: Option<&str> = Some("database_url");

    match config_value {
        Some(value) => { println!("The configuration value is : {}", value) },
        None => { println!("The configuration value is not set.")}
    }

    let another_config_value: Option<&str> = None;
    match another_config_value {
        Some(value) => {
            println!("This won't be printed {}", value)
        },
        None => {
            println!("This will be printed because the value is None")
        }
    }

    let numbers1 = vec![1,3,5,7,9];
    let numbers2 = vec![2,4,6,8,10];

    let first_even1 = find_first_even(&numbers1);
    let first_even2 = find_first_even(&numbers2);

    println!("First even in numbers 1: {:?}", first_even1 );
    println!("First even in numbers 2: {:?}", first_even2 );




    let circle = Shape::Circle(10.0, 15.0, 5.0);
    let rect = Shape::Rectangle(0, 0, 100, 200);
    let point = Shape::Point;

    print_shape_details(&circle);
    print_shape_details(&rect);
    print_shape_details( &point);
}

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num)
        }
    }
    None
}

fn print_shape_details(shape: &Shape) {
    match shape {
        Shape::Circle(x, y, radius ) => println!("Circle at ({}, {}) with radius {}", x,y,radius),
        Shape::Rectangle(x, y,  width, height) => println!("Rectangle at ({}, {}) with width {} and height {}", x, y, width, height),
        Shape::Point => println!("Just a point"),
    }
}