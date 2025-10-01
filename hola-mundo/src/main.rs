fn main(){
    let numeros = vec![1,2,3,4,5,6];
    
    let suma = numeros
        .iter()
        .fold(0, |acumulador, &x| acumulador + x);

    println!("La suma es {}", suma); 
}