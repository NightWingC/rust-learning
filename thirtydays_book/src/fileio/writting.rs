use std::fs::File;
use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut file = File::create("output.txt")?;
    let data_to_write = "This is the first line\n This is the second line\n";
    let bytes_to_write = data_to_write.as_bytes();

    file.write_all(bytes_to_write)?;
    println!("Successfully wrote to output.txt");
    Ok(())
}


// example 2
// use std::fs::File;
// use std::io::prelude::*;
// use std::io;

fn main() -> io::Result<()> {
    let mut file = File::create("another_output.txt")?;

    writeln!(file, "Hello from rust!");
    writeln!(file, "This is line number 2");
    let number = 42;
    writeln!(file, "The answer is: {}", number);
    println!("Successfully wrote lines to another_output.txt");
    Ok(())
}

// Example 3
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("append_example.txt")?;

    writeln!(file, "This line will be appended");
    writeln!(file, "Another line to append." );

    println!("Successfully append to append_example.txt");
    Ok(())
}