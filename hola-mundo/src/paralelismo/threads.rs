use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5{
            println!("Hola desde el hilo secundario: {}",i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..5 {
        println!("Hola desde el hilo principal: {}",i);
        thread::sleep(Duration::from_millis(300));
    }

    handle.join().unwrap();
}