fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

struct Parts<'a>{
    m: &'a str,
    n: &'a str,
}

impl <'a> Parts <'a> {
    fn x(&self) -> &'a str {
        self.m
    }

    fn y(&self) -> &'a str {
        self.n
    }

    fn longest(&self) -> &'a str {
        if self.m.len() > self.n.len() {
            self.m
        } else {
            self.n
        }
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()  {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


fn main(){
    let my_string = String::from("hello world"); 
    let word = first_word(&my_string[0..6]); 
    println!("The first word is: {}", word); 

    let my_string_literal = "hello world again"; 
    let word_literal = first_word(my_string_literal);
    println!("The first word is: {}", word_literal);
    
    // 

    let s : &'static str = "I have a static lifetime";
    println!("{}",s); 
    // 

    let ss1 = String::from("abcd"); 
    let ss2 = String::from("xyz");

    let parts = Parts {
        m: ss1.as_str(),
        n: ss2.as_str(),
    };

    // 

    let longest_part = parts.longest();
    println!("The longest part is {}", longest_part);

    let novel = String::from("Call me Ismael. Some year ago...");
    let i = novel.len() - 3;
    let first_sentence = ImportantExcerpt {
        part: novel.as_str(),
    };

    println!("Excerpt: {}", first_sentence.part);

    // 
    let s1 = String::from("abcd"); 
    let s2 = String::from("xyz");

    let result = longest(s1.as_str(), s2.as_str());
    println!("The longest string is {}", result);

    // 
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);

    let string3 = String::from("long string is long");
    let result2 = longest(string1.as_str(), string3.as_str());

    println!("The longest string is {}", result2);

   
}