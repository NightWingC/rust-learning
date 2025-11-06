fn parse_string_to_int(s: &str) -> i32 {
    match s.parse() {
        Ok(number) => {
            println!("Successfully parsed: {}", number);
            number
        }
        Err(e) => {
            eprintln!("Failed to parse string'{}': {}", s, e);
            0
        }
    }
} 

fn parse_string_with_if_let(s: &str) -> i32 {
    if let Ok(number) = s.parse::<i32>(){
        println!("Successfully parsed via if let: {}", number);
        number 
    } else {
        eprintln!("Failed to parse string '{}', using if let Using default", s);
        0
    }
}

fn main() {
    let valid_string2 = "456";
    let invalid_string2 = "def";

    let num2_1 = parse_string_with_if_let(valid_string2);
    println!("Result for '{}': {}", valid_string2, num2_1 );

    let num2_2 = parse_string_with_if_let(invalid_string2);
    println!("Result for '{}': {}", num2_2, invalid_string2);

    let valid_string = "123";
    let invalid_string = "abc";

    let num1 = parse_string_to_int(valid_string);
    println!("Result for '{}': {}", valid_string, num1);


    let num2 = parse_string_to_int(invalid_string);
    println!("Result for '{}': {}", invalid_string, num2);
}