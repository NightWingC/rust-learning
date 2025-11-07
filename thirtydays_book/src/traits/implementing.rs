trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify<T: Summary>(item: &T){
    println!("Breaking news! {}", item.summarize())
}

trait PrettyPrint {
    fn display(&self) -> String {
        String::from("Default display format")
    }
}

struct User {
    name: String,
    age: u32,
}

impl PrettyPrint for User {}

struct Product {
    id: i32,
    name: String,
}

impl PrettyPrint for Product {
    fn display(&self) -> String {
        format!("Product Id: {}, Name: {}", self.id, self.name)
    }
}


fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 30
    };

    let product = Product {
        id: 20,
        name: String::from("Soup"),
    };

    println!("{}", user.display());
    println!("{}", product.display());

    println!("--------------------------------");

    let article = NewsArticle {
        headline: String::from("Penguins win the stanley cup championship!"),
        author: String::from("Iceburgh"),
        location: String::from("Pittsburgh, PA, USA"),
        content: String::from("The Pittsburg Penguins once again are the best team in the NHL"),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you probably already know, people"),
        retweet: false,
        reply: false
    };

    notify(&article);
    notify(&tweet);
}