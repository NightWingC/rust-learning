fn main() {
    let data_slice = [10, 20, 30, 40, 50];
    let [a, b, rest @ ..] = data_slice;
    println!("a: {}, b: {}, rest: {:?}", a, b, rest);

    let data = [10, 20, 30, 40, 50];
    let [first_val, _,_,_, last_val] = data;
    println!("First: {}, Last: {}", first_val, last_val);

    let numbers = [1, 2, 3, 4, 5];
    let [first, second, .., last] = numbers;
    println!("First: {}, Second: {}, Last: {}", first, second, last);

    let coordinates = (10.2, 20.3, 30.4);
    let (x, y, _) = coordinates;
    println!("X: {}, Y: {}", x, y);

    let user_info = (String::from("Alice"), 30 , String::from("New York"));

    let (name, age, city) = user_info;
    println!("Name: {}, Age: {}, City: {}", name, age, city);

}