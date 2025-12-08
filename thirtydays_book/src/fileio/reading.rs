use std::fs;
use std::io;

fn main() -> io::Result<()> {
   let file_path = "hello.txt";
   match fs::read_to_string(file_path) {
        Ok(contents) => {
            println!("File content: \n{}", contents);
        },
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            match error.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("The file {} was not found", file_path);
                }
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Permission denied to read the file {}", file_path);
                }
                _ => {
                    eprintln!("An unexpected I/O error occurred: {}", error);
                }
            }
        },
    } 
    Ok(())
}

// Second example 
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::fs;

fn read_file_lineby_line(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    println!("Reading file {} line by line", file_path);
    for (index, line_result) in reader.lines().enumerate()  {
        match line_result {
            Ok(line) =>  {
                println!("Line {}: {}", index + 1, line);
            }
            Err(error) => {
                eprintln!("Error reading line {}: {}", index + 1, error);
            }
        }
    }

    Ok(())
}

fn main() {
    let file_to_read = "sample_dat.txt";
    if let Err(e) = fs::write(file_to_read, "First line of data. \nSecond line \n Third and final line") {
        eprintln!("Error creating sample file: {}", e);
        return;
    }

    if let Err(e) = read_file_lineby_line(file_to_read) {
        eprintln!("An error occured during the file reading process: {}", e);
    }
}