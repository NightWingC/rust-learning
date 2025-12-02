use std::io::{self, Read};

fn main() -> io::Result<()> {
    println!("Enter some text. This will be read as raw bytes");
    println!("Press Ctrl+ D (unix) or Ctrl + Z then Enter (Windows) to signal end of input");
    let mut buffer = [0u8, 10];
    println!("Reading in chunks of {} bytes...", buffer.len());

    loop {
        let bytes_read = io::stdin().read(&mut buffer)?;
        if bytes_read == 0 {
            println!("End of input stream reached");
            break;
        }

        let data_read = &buffer[..bytes_read];
        print!("Read {} bytes:", bytes_read);
        for byte in data_read {
            print!("{:02X} ", byte);
        }

        println!();
        if bytes_read > 0 && data_read[0] == b'\r' {
            println!("Encountered a carriage return as the first byte. Stopping.");
            break;
        }

    }

    Ok(())
}