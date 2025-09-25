fn main(){
    let numeros = 0..=5;
    let otros_numeros = numeros.clone();

    for num in numeros{
        println!("consumi el numero: {num}");
    }

    for num2 in otros_numeros {
        println!("Consumí el numero: {num2}");
    }
}