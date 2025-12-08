use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("New tread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// Example 2
use std::thread;

fn main() {
    let message = String::from("Helloo from the main thread!");
    let handle = thread::spawn(move || {
        println!("Here's the message: {}", message);
    });
    handle.join().unwrap();
}