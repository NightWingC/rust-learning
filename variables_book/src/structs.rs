struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;


fn main(){
    let mut user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("anotherexample@example.com"),
        ..user1
    };

    let black = Color(0,0,0);
    let piont  = Point(0,0,0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, 
        email,
        sign_in_count: 2
    }
}