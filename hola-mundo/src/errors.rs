// Like it looks
// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::{Read, Result};

fn main(){
    println!("Divisor 1: {:?}", dividir(3.0, 2.0));
    println!("Divisor 2: {:?}", dividir(3.0, 0.0));

    let resultado = dividir(10.0, 0.0);
    match resultado {
        Ok(valor) => println!("Resultado: {}", valor),
        Err(error) => eprintln!("Error: {}", error)
    }

    if let Ok(valor) = dividir(10.0, 0.0) {
        println!("Resultado: {valor}")
    } else {
        println!("Hubo un error")
    }
}

fn dividir(dividendo: f64, divisor: f64) -> Result<f64, String>{
    if divisor == 0.0 {
        Err(String::from("No se puede dividir entre cero"))
    } else {
        Ok( dividendo / divisor )
    }
}


// Leer un archivo

fn leer(){
    match leer_archivo("test.txt") {
        Ok(texto) => println!("Contenido \n {texto}"),
        Err(e) => eprintln!("Error al leer el archivo: {e}"),
    }
}

fn leer_archivo(path: &str) -> Result<String> {
    let mut archivo = File::open(path)?;
    let mut contenido = String::new();
    archivo.read_to_string( &mut contenido)?;
    Ok(contenido)
}


 
#[derive(Debug)]

enum MiError {
    DivisionCero,
    IO(std::io::Error),
    Otra(String),
}

fn dividir_dos(dividendo: f64, divisor: f64) -> Result<f64, MiError>{
    if divisor == 0.0 {
        Err(MiError::DivisionCero)
    } else {
        Ok(dividendo / divisor)
    }
}
fn mi_error(){
    println!("resultado: {:?}", dividir_dos(2.0, 0.0));
}


// unwrap and expect 
use std::fs::File;
use std::io;

fn abrir_archivo(path: &str) -> io::Result<File> {
    File::open(path)
}

fn unwrap_expect(){
    let archivo = abrir_archivo("datos.txt").unwrap();
    println!("Archivo abierto: {:?}", archivo);

    let archivo_dos = abrir_archivo("datos2.txt").expect("No se pudo abrir el archivo");
    println!("Archivo 2 abierto: {:?}", archivo_dos);

}

