use std::io::{self, Read};

fn main() {
    println!("Enter a sequence of charaters ending with. a semicolon(;):");

    let mut buffer2 = Vec::new();

    match io::stdin().read_until(b';', &mut buffer2) {
        Ok(bytes_read) => {
            let input_str = String::from_utf8_lossy(&buffer2);
            println!("Read {} bytes. Input invluding delimiter {}", bytes_read, input_str);
        }
        Err(error) => {
            eprintln!("Error reading from stdin: {}", error);
        }
    }


    println!("Enter a few characters:");
    let mut buffer = [0u8, 10];
    match io::stdin().read(&mut buffer) {
        Ok(bytes_read) => {
            let input_str = String::from_utf8_lossy(&buffer[..bytes_read]);

            println!("Read {} bytes. Input {}", bytes_read, input_str.trim());
        }

        Err(error) => {
            eprintln!("Error reading from stdin: {}", error);
        }
    }

    println!("Please enter something:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)
    .expect("Failed to read line");

    println!("You entered: {}", input_string);
}