use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mensajes = vec!["Hola", "desde", "el", "hilo"];
        for msg in mensajes {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for recibido in rx {
        println!("Recibí :{}", recibido);
    }
}