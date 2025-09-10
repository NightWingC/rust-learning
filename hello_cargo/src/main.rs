// use rand::random;
// const NRO_ESPACIOS:i32 = 10;
// Tipado estático: chequeo de los tipos de datos se hace en tiempo de compilación
// fn main() {
//     let espacios = "   ";
//     println!("El usuario ingreso: {}", espacios);
//     let espacios = espacios.len();
//     println!("El numero de espacios: {}",espacios);
//     println!("Numero de estpacios por defecto: {}", NRO_ESPACIOS);
// }

fn main() {
    // INTEGER
    let entero: i8 = 23;
    let entero2: u8 = 40;
    let entero2: i8 = -40; 

    // Integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;

    // Floating
    let float1 = 5.0; // f64
    let floa32: f32  = 12.4320;

    // Boolean
    let verdadero = true;
    let falso: bool = false;

    // Character
    let caracter = "a";
    let emoji = 'a';

    // Compuestos types
    // Tuplas
    let tupla = ("h", 23, -45, 0.223);
    // let tupla2: (char, i64, i32, f64 ) = ("h", 23, -45, 0.223);

    let (x, y , z,w) = tupla;
    println!("El valor de de x es: {}", x);
    println!("El segundo valor de la tupla es: {}", tupla.1);

    // Array
    let arreglo = [1,2,3,4,5];
    println!("El segundo valor del arreglo es: {}", arreglo[1]);

    let arreglo2: [i128;5] = [1,2,3,4,5];

    // strings
    // String slide
    let mut nombre:  = "Julio Andres";
    nombre = "Pepe";

    let apellido: String = "Julio".to_string();
    let mut domicilio: String::new();
    domicilio = "mi casa".to_string();

    let strings = "adadas";


}



