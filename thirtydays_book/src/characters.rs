fn main() {
    let new_line = "\n";
    let tab = "\t";
    let backspace= "\x08";
    let unicode_char = "\u{1F4A9}";

    println!("Newline: {}", new_line);
    println!("Tab: {}", tab);
    println!("Backspace is invisible here: {}", backspace);
    println!("Unicode char: {}", unicode_char);
    
}