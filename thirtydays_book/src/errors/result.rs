use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let f = File::open("usernam.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_concise() -> Result<String, io::Error> { 
    let mut f = File::open("username.txt")?; // If Err, return Err(e) from this function 
    let mut s = String::new(); 
    f.read_to_string(&mut s)?; // If Err, return Err(e) from this function Ok(s) // If both operations succeed, return 
    Ok(s) 
}