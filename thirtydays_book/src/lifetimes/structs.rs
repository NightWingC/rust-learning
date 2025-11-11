struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("abcd"); 
    let string2 = String::from("xyz");

    let result;

    {
        let novel = String::from("Call me Ismael. Some years ago..");
        let excerpt = ImportantExcerpt {
            part: &novel
        };

        println!("Excerpt part: {}", excerpt.part)
    }

    let string_data = String::from("This is a long string that will live for a while."); 
    let excerpt_valid = ImportantExcerpt { 
        part: &string_data[0..10], 
     }; 
     println!("Valid excerpt part: {}", excerpt_valid.part);

}