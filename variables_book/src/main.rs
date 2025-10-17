<<<<<<< HEAD
// Spoiler alert
fn main(){
    let mut s = String::from("Hello");
    change(&mut s);
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
=======
enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
>>>>>>> 489b48c (concurrency)
}