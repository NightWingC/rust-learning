
fn main() {
    // Control flow
    // if expressions
    let number = 7;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false ")
    }

    if number != 0 {
        println!("Number was something other than zero")
    }

    // Handling multiple conditions with else if 
    let num = 6;
    if num % 4 == 0 {
        println!("Number is divisible by 4");
    } else if num % 3 == 0 {
        println!("Number is divisible by 3");
    } else if num % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // Using if in let statement 
    let condition = false;
    let num_two = if condition { 5 } else { 6 };
    println!("The value of number is: {num_two}");

    // Repetition with loops
    // Repeating code with loop
    loop {
        println!("again!");
        break;
    }

    // Retunrning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {result}");

    // Loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up:loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; 
            }

            remaining -=1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
