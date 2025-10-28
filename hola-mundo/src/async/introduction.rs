use tokio::time::{ sleep, Duration };

async fn saludar(id: u8){
    println!("Tarea {}: iniciada", id);
    sleep(Duration::from_millis(500)).await;
    println!("Tarea {}: completada", id);
}

#[tokio::main]
async fn main() {
    println!("Inicio de main asincrono");
    saludar(1).await;
    saludar(2).await;

    let t3 = saludar(3);
    let t4 = saludar(4);

    tokio::join!(t3,t4);

    println!("Todas las tareas han finalizado");
}

