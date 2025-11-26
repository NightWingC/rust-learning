// Diferent name 
struct Point {
    x: i32,
    y: i32,
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data");
        }

        Message::Move { x, y } => {
            println!("The move variant moves to x: {}, y:{}", x, y);
        }

        Message::Write(text) => {
            println!("The Write variant contains this text: {}", text);
        }

        Message::ChangeColor(r, g , b) => {
            println!("The ChangeColor variant contains r: {}, g: {}, b: {}", r, g, b);
        }
    };

    let user = User {
        email: String::from("someone@examplo.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    match user {
        User { username, active: true, .. } => {
            println!("Username: {}, is active", username);
        }

        User { username, .. } => {
            println!("{} is inactive", username);
        }
    }

    let p = Point { x: 10, y: 20 };
    match p {
        Point {x: new_x, y: new_y} => {
            println!("x: {}, y: {}", new_x, new_y);
        }

    }
}