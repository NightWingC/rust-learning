
fn main() {
    // Functions
    // Parameters
    another_function(5);
    print_labaled_measurement(5, 'h');

    // Statements and expressions
    let x = 6;
    let y = {
        let x = 3;
        x + 1;
    };

    // println!("the value of Y is: {}", y);
    let five = five();
    println!("Return five fn: {five}");

    let sum = plus_one(5);
    println!("Sum: {sum}");
}

fn plus_one(num: i32) -> i32 {
    num + 1
}

fn another_function(x: i32){
    println!("The value of X is: {x}");
}

fn print_labaled_measurement( value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}")
}

fn five() -> i32 {
    5
}