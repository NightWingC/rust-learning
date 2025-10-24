fn main() {
    let s1 = String::from("Value");
    let len = calculate_lenght(&s1);

    println!("The length of '{}' is {}", s1, len);

    let r1 = &s1;
    let r2 = &s1;

    println!("{} and {}", r1, r2  );

    let mut x = 5;
    let y = &x;
    let z = &mut x;

    *z += 1;

    println!("the value of x is : {}", x);
    let mut my_number = 10;
    println!("Before calling add one: {}", my_number);

    add_number(&mut my_number);
    println!("after calling add one: {}", my_number);

    let mut v = vec![1,2,3,4,5];
    let first = &mut v[0];

    *first += 10;
    println!("modified first element: {}", first);

    let second = &v[1];
    println!("The second element is : {}", second);

    let mut s1 = String::from("H3llo");
    let rs1 = &s1;
    println!("rs1 : {}", rs1);

    let rs2 = &mut s1;
    rs2.push_str(", world!");
    println!("rs2: {}", rs2);

}

fn add_number(number: &mut i32){
    *number += 1;
}

fn calculate_lenght(s: &String) -> usize{
    s.len()
}