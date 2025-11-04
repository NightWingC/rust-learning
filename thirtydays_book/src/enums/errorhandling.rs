
use std::fs::File;
use::std::io::{self, Read};

fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // If opening the fails, return io:Error

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;// If reading the file fails, return io::Error

    Ok(contents) // on success, return the file content as an Ok value
}

fn main(){

    let result: Result<String, io::Error> = Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));

    let mapped_err_result = result.map_err(|err| err.to_string());
    println!("Mapped error result: {:?}", mapped_err_result);

    let success_result: Result<i32, &str> = Ok(10);
    let failure_result: Result<i32, &str> = Err("Something went wrong");

    // Successfully extract the Ok value
    let value = success_result.unwrap();
    println!("Unwrapped value: {}", value);

    let expected_value = failure_result.expect("Expected a successful result, but got an error!");

    let value1 = success_result.unwrap_or(0);
    println!("Unwrapped or default (success): {}", value1);

    let value2 = failure_result.unwrap_or(0);
    println!("Unwrapped or default (failure): {}", value2);
    
    match read_file_content("my_file.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error reading file: {}", error),
    }
}
