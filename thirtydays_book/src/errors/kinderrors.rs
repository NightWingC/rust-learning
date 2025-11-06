use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

// a custom error type to wrap different kinds of errors
#[derive(Debug)]
enum AppError {
    Io(io::Error), 
    Parse(ParseIntError),
}

// Implement from for io::Error to convert it to AppError
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::Io(err)
    }
}

// implement from for parseinterror to convert it to appError
impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> AppError {
        AppError::Parse(err)
    }
}

fn read_and_parse_number(filename: &str) -> Result<i32, AppError> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse::<i32>()?;
    Ok(number)
}

fn main() {
    // Create a dummy file for testiing
    if let Err(e) = std::fs::write("number.txt", "12345"){
        eprintln!("Failed to create dummy file: {}", e);
        return;
    }

    match read_and_parse_number("number.txt") {
        Ok(num) => println!("Successfully read and parsed number: {}", num),
        Err(AppError::Io(e)) => eprintln!("An I/O error occurred: {}", e),
        Err(AppError::Parse(e)) => eprintln!("A parsing error occured: {}", e),        
    }

    let _ = std::fs::remove_file("number.txt");

    println!("\nAttemping to read from a non-existent file:");

    match read_and_parse_number("nonexistent.txt") {
        Ok(num) => println!("Successfully read and parsed number: {}", num), 
        Err(AppError::Io(e)) => eprintln!("An I/O error occurred: {}", e), 
        Err(AppError::Parse(e)) => eprintln!("A parsing error occurred: {}", e),
    }

    if let Err(e) = std::fs::write("invalid_number.txt", "not a number") { 
        eprintln!("Failed to create invalid content file: {}", e); 
        return; 
    }

    println!("\nAttempting to read invalid content:");

    match read_and_parse_number("invalid_number.txt") { 
        Ok(num) => println!("Successfully read and parsed number: {}", num), 
        Err(AppError::Io(e)) => eprintln!("An I/O error occurred: {}", e), 
        Err(AppError::Parse(e)) => eprintln!("A parsing error occurred: {}", e), 
    } 
    
    let _ = std::fs::remove_file("invalid_number.txt");

}