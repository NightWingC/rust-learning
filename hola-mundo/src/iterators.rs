fn iters(){
    let numeros = vec![10,20,30,40];
    let mut iterador = numeros.iter();

    println!("{:?}", iterador.next());
    println!("{:?}", iterador.next());
    println!("{:?}", iterador.next());
    println!("{:?}", iterador.next());
    println!("{:?}", iterador.next());


    let mut nombres = vec!["ana", "bob", "ford"];
    for nombre in nombres.iter_mut(){
        println!("Hola {nombre}")
    }  
}

fn maps(){
    let numeros = vec![1,2,3,4];
    let iter_cuadrados = numeros.iter().map(|x| x * x);

    let cuadrados : Vec<i32> = iter_cuadrados.collect();

    println!("Original: {:?}", numeros);
    println!("Cuadrados: {:?}", cuadrados);
}


fn filters(){
    let numeros = vec![1,2,3,4,5,6];
    let iter_pares = numeros.iter().filter(|&x| x % 2 == 0);
    let iter_impares = numeros.iter().filter(|&x| x % 2 != 0);

    let impares : Vec<i32> = iter_impares.cloned().collect();
    let pares : Vec<i32> = iter_pares.cloned().collect();

    println!("Impares: {:?}", impares);
    println!("Pares: {:?}", pares);


    let resultado: Vec<_> = numeros
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("Pares al cuadrado: {:?}", resultado);
}

fn folds(){
    let numeros = vec![1,2,3,4,5,6];
    
    let suma = numeros
        .iter()
        .fold(0, |acumulador, &x| acumulador + x);

    println!("La suma es {}", suma); 
}