fn main() {
    // declaring an array of 5 element of type i32
    let a: [i32; 5] = [1,2,3,4,5];
    println!("The array is: {:?}", a);

    // declaring an array of 10 elements of type i32, all initialized to 0
    let b = [0;10];
    println!("The initialixed array is: {:?}", b);

    // Accesing the first element(index 0)
    let first_element = a[0];
    println!("Fisrt element array a is: {}", first_element);

    // This line wil case a panic beacuse index 10 is out of bounds for an array of size 
    // let element = a[10];
    // println!("Error array element, {}", element);

}