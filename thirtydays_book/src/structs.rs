struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct  Color (i32, i32, i32);
struct  Point (i32, i32, i32);

struct Point3D(f64,f64,f64);

struct AlwaysBeTalking;

struct ErrorAlert;
struct WarningAlert;
struct InfoAlert;

struct NoOpChannel;

struct SilentLogger;

struct Rectangle {
    width: u32,
    height: u32,
}

trait Notifiable {
    fn send_notification(&self);
}

trait Logger {
    fn log(&self, message: &str);
}

impl Notifiable for ErrorAlert {
    fn send_notification(&self) {
        println!("Sending a critiacal error notification");
    }
}

impl Notifiable for WarningAlert {
    fn send_notification(&self) {
        println!("Sending a warning notification")
    }
}

impl Notifiable for InfoAlert {
    fn send_notification(&self) {
        println!("Sending a informational notification")
    }
}

impl Notifiable for NoOpChannel {
    fn send_notification(&self) {
        println!("No operation performed")
    }   
}

impl Logger for SilentLogger {
    fn log(&self, message: &str){

    }
}

impl Rectangle  {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn scale(&mut self, factor: u32 ){
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    
    let sq = Rectangle::square(10);

    println!("The area of the square is {} square pixels", sq.area());

    let mut rectangle1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangule is {} square pixel.", rectangle1.area());

    println!("Original rectangule: width = {}, height = {}", rectangle1.width, rectangle1.height);
    rectangle1.scale(2);
    println!("Scale rectangle: width = {}, height = {}", rectangle1.width, rectangle1.height);
    println!("The new area of the scaled rectangle is: {}", rectangle1.area());

    let logger = SilentLogger;
    logger.log("This message wil not be seen");

    let error = ErrorAlert;
    error.send_notification();

    let warning = WarningAlert;
    warning.send_notification();

    let info = InfoAlert;
    info.send_notification();

    let no_op = NoOpChannel;
    no_op.send_notification();

    let a_point = Point3D(3.0, 4.0, 0.0);
    let dist = distance_from_origin(a_point);


    let subject = AlwaysBeTalking;

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("The origin x-coordinate is: {}", origin.0);
    println!("The black color's blue component is: {}", black.0);


    let user1 = User {
        username: String::from("someuser"),
        sign_in_count: 1,
        active: true,
        email: String::from("someuser@mail.com"),
    };

    let user2 = User {
        username: String::from("anotheruser"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
        email: String::from("another@mail.com"),
    };

    let user3 = User {
        username: String::from("otheruser"),
        email: String::from("other@mail.com"),
        ..user1
    };

}

fn distance_from_origin(point: Point3D) -> f64 {
    (point.0.powi(2) + point.1.powi(2) + point.2.powi(2)).sqrt()
}