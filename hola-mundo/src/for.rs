fn main(){
    let numeros = vec![10,20,30];

    for num in &numeros{
        println!("El numero es: {num}");
    }
    println!("Numero es = {}", numeros[1]);


    // Moviendo la prooiedad, desaparece de memoria 
    for num in numeros{
        println!("Consumi el numero: {num}");
    }

    // println!("Numero es = {}", numeros[1]);

    for i in 0..=5 {
        println!("i = {i}");
    }
}