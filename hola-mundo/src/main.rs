use std::fs::File;
use std::io;

fn abrir_archivo(path: &str) -> io::Result<File> {
    File::open(path)
}

fn main(){
    let archivo = abrir_archivo("datos.txt").unwrap();
    println!("Archivo abierto: {:?}", archivo);

    let archivo_dos = abrir_archivo("datos2.txt").expect("No se pudo abrir el archivo");
    println!("Archivo 2 abierto: {:?}", archivo_dos);

}

