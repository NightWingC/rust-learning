fn main() {
    // len()
    //  
    let my_string = String::from("Hello world example");
    let word = first_word(&my_string);
    println!("The first word is: {}", word);

    let my_string_literal = "Another example";
    let word_from_literal = first_word(my_string_literal);
    println!("The first world is: {}", word_from_literal);

    let s = String::from("Hello world");
    let slide = &s[0..5];
    println!("{}", slide);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}