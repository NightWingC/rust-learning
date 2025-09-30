fn main(){
    println!("El cuadrado de 2 es {}", cuadrado(2));
}

fn cuadrado(n: i32) -> (i32, str) {
    (n * n, "hola")
}


struct Rango {
    min: i32,
    max: i32,
}
fn main_example(){
    let nums = vec![10, 3, 5 ,7,2];
    let rango = min_max_structs(&nums);
    println!("El menor es: {} y el mayor es {}", rango.min, rango.max);
    match obtener_elemeto(&nums, 10) {
        Some(valor) => println!("El valor encontrado es: {valor}"),
        None => println!("Indice Invalido")
    }
}

fn min_max_structs(valores: &[i32]) -> Rango { 
    let mut min = valores[0];
    let mut max = valores[0];
    for &x in valores {
        if x < min { min = x }
        if x > max { max = x }
    }
    Rango { min, max }
}

fn obtener_elemeto(vec: &[i32], indice: usize) -> Option<i32> {
    if indice < vec.len() {
        Some(vec[indice])
    } else {
        None
    }
}   