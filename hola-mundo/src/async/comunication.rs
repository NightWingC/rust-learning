// src/main.rs
use tokio::{
    sync::{ mpsc, oneshot },
    time::{ sleep, Duration },
};

/// Productor: envía números del 1 al 10 cada 200ms
/// Se detiene si recibe señal por shutdown_rx.
async fn productor(
    tx: mpsc::Sender<i32>,
    mut shutdown_rx: oneshot::Receiver<()>, 
){
    for i in 1..=10 {
        //Chequeo de shutdown sin bloquear
        if shutdown_rx.try_recv().is_ok() {
            println!("Productor: Recibida la señal de parada");
            break;
        } 

        // Enviar mensaje (await si el buffer esta lleno)
        tx.send(i).await.unwrap();
        println!("Productor envió: {}", i);
        sleep(Duration::from_millis(200)).await;
    }

    println!("Productor cerrando el canal");
}

// Consumidor recibe numeros hasta que el canal se cieer
async fn consumidor( mut rx: mpsc::Receiver<i32>) {
    while let Some(value) = rx.recv().await {
        println!("Consumidor recibió: {}", value);

        sleep(Duration::from_millis(300)).await;
    }

    println!("Consumidor el canal se ha cerrado, terminado");
}

#[tokio::main]
async fn main() {
    println!("Inicio de comunicaciones");
    let (tx, rx) = mpsc::channel(5);
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    // Lanzamos productor y consumidor
    let prod_handle = tokio::spawn(productor(tx, shutdown_rx));
    let cons_handle = tokio::spawn(consumidor(rx));

    // Despues de 2 segundos, enviamos la señal de shutdown.
    tokio::spawn(async move {
        sleep(Duration::from_secs(2));
        let _ = shutdown_tx.send(());
        println!("Señal de shutdown emviada");
    });

    prod_handle.await.unwrap();
    cons_handle.await.unwrap();
    println!("Fin del capitulo");
}