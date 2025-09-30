fn main(){
    let suma = | a: i32, b: i32 | -> i32 {
        a + b
    };

    let resultado = suma(5 ,3);
    println!("La suma es: {resultado}");
}

fn main_example(){
    let mut contador = 0;
    let mut incrementar = || contador += 1;

    incrementar();
    incrementar();

    println!("Contador: {contador}");

    let texto = String::from("Hola Rust");
    let imprimir = move || {
        println!("{texto}");
    };

    // println!("{texto}");
    imprimir();
}

// Funciones dentro de funciones 
fn main_example_two(){
    let precios = vec![100.0, 80.0, 60.2];
    let con_impuestos = procesar_lista(&precios, |x| x * 1.10);
    println!("Con impuesto: {:?}", con_impuestos);

    let con_descuento = procesar_lista(&precios, |x| x - 5.0);
    println!("Con impuesto: {:?}", con_descuento);

    fn redondear(x: f64) -> f64 {
        x.round()
    }

    let redondeados = procesar_lista(&precios, redondear);
    println!("Redondeados: {:?}", redondeados);

}

fn procesar_lista<F>(datos: &[f64], transformacion: F) -> Vec<f64> 
    where F: Fn(f64) -> f64,
    {
        let mut resultado = Vec::with_capacity(datos.len());
        for &valor in datos {
            let nuevo = transformacion(valor);
            resultado.push(nuevo);
        }
        resultado
    } 

    // Funciones orden superior 
fn doble(x: i32) -> i32 { x * 2}
fn mas_cinco(x: i32) -> i32 { x + 5}

fn comp<F,G>(f: F, g: G) -> impl Fn(i32) -> i32 
    where 
        F: Fn(i32) -> i32,
        G: Fn(i32) -> i32,
        {
            move| x | g(f(x)) 
        }
fn main_example_three(){
    let doble_mas_cinco = comp(doble, mas_cinco);
    println!("Resultado: {}", doble_mas_cinco(3))
}
