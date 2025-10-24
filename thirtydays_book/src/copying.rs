#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
fn main(){
    let s = String::from("value");
    takes_ownership(s);
    // println!("Printing s: {}", s); // This would cause a compile-error 

    let s1 = gives_ownership();
    let s2 = String::from("Value");
    let s3 = takes_and_gives_back(s2);
    
    // println!("{}", s2); // this would cause a compile-time error
    println!("s1: {} , s2: {}", s1, s3);
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1;

    println!("P1 = {:?}, P2 = {:?}", p1, p2);

    fn move_point(p: Point){
        println!("Moved point: {:?}", p);
    }

    move_point(p1);
    println!("P1 after function call: {:?}", p1);

}

fn gives_ownership() -> String {
    let some_string = String::from("Yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}