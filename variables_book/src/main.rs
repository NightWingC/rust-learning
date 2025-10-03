fn main(){

    let mut number = 3;
    
    while number != 0 {
        println!("number: {number}");
        number -=1;
    }
    println!("LIFTOPFF!!");

    let a = vec![10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("The values is {}", a[index]);
        index += 1;
    } 
    println!("LIFTOPFF!!");


    for num in a {
        println!("The values is {num}");
    }
    println!("LIFTOPFF!!");


    // Rev
    for number in (1..=4).rev(){
        println!("The number is: {number}");
    }
    println!("LIFTOPFF!!");

}