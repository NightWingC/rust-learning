// enum Option<T> {
//     None,
//     Some(T),
// }

#[derive(Debug)]
struct Usuario {
    nombre: String,
    edad: u8,
}

fn main() {
    let valor = Some(42);
    let texto = Some(String::from("Hola"));
    match valor {
        Some(numero) => println!("El numero es: {numero}"),
        None => println!("No hay numero")
    }

    match texto {
        Some(nombre) => println!("El numero es: {nombre}"),
        None => println!("Ingrese un texto")
    }


    let usuario = Some(Usuario{
        nombre: String::from("Pablo"),
        edad: 54,
    });

    match usuario {
        Some(u) =>println!("El usuario es: {:?}", u),
        None => println!("No hay usuario"),
    }

}
