fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}."); 
    println!("{s1}");

    let mut s = String::from("Hello");
    
    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}");

    let r3 = &mut s;

    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main() {
    let mut s = String::from("Hello world");
    let word = first_word(&s);
     //s.clear(); // this empties the string, making it equal to ""
    println!("The first word is:  {word}");

    let my_string = String ::from("Hello world");


}

fn first_word(s: &String) -> &str { 
    let bytes = s.as_bytes(); 
    for (i, &item) in bytes.iter().enumerate() { 
        if item == b' ' { 
            return &s[0..i]; 
        } 
    }

    &s[..]
}


