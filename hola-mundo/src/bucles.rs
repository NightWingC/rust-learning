
fn main() {
    let mut counter = 0;

    let resultado = loop {
        counter += 1;
        println!("Contando: {counter}");

        if counter == 5 {
            break "Alcance el 5"; 
        }
    };

    println!("El loop regresó {}", resultado);

    let mut n = 3;
    while n !=0 {
        println!("N vale: {n}");
        n -= 1
    }

    println!("Fin del bucle while");


    let mut numeros = vec![1,2,3];
    while let Some(valor) = numeros.pop(){
        println!("Obtuve: {}", valor);
    }
}

