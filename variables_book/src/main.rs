
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
}

fn another_function(x: i32){
    println!("The value of X is: {x}");
}

fn print_labaled_measurement( value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}")
}