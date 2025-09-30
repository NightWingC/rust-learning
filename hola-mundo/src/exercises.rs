fn main(){
    let mut count = 0;
    while count <= 5 {
        println!("count: {count}");
        count += 1;
    };

    let mut n = 1; 
    let valor_final = loop {
        println!("n: {n}");
        n *= 2; 
        if n >= 32 { 
            break n;
        }  
    };
    
    println!("valor final: {valor_final}");

    // Rango inclusivo
    for num in 1..=10 {
        if num % 2 == 0 {
            println!(" {num} es par");
        } else {
            println!("{num} es impar");
        }
    }

}