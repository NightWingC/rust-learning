// src/main.rs
use tokio::fs;
use tokio::io;

async fn file_io_example() -> io::Result<()> {
    let path = "texto_async.txt";
    let contenido = "Hola desde tokio async I/O";

    // Escribe el archivo (suspende sin bloquear)
    fs::write(path, contenido).await?;
    println!("Archivo escrito con {}", contenido);

    // Lee el archivo completo de forma asincrona
    let texto = fs::read_to_string(path).await?;
    println!("Contenido leído: {}", texto);
    Ok(())
}
#[tokio::main]

async fn main(){
    println!("Inicio de escritura y lectura de disco");
    if let Err(e) = file_io_example().await {
        println!("Error en el file I/O {}", e);
    }

    println!("Fin de escritura y lectura de disco");
}