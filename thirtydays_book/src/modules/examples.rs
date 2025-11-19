mod greetings;
mod calculator;
mod geometry;
mod colors;


fn main() {
    // libs 
    let conn = crate::networking::protocols::TcpConnection::new("192.168.1.1");

    let message = b"Hello, server!";
    match crate::networking::protocols::send_data(&conn, message) {
        Ok(_) => println!("Data sent successfuky"),
        Err(e) => eprintln!("Error sending data: {}",e),
    }
    // 

    let red = colors::Color::Red;
    let rgb = colors::Color::RGB(255, 255, 255);

    print_color(red);
    print_color(rgb);
    // 
    let p = geometry::Point::new(10, 20);
    println!("X coordinate: {}", p.x);
    println!("Y coordinate: {}", p.y);

    let (x, y) = p.get_coords();
    println!("Point coordinates:({},{})", x, y);
    
    // 
    let sum = calculator::add(10,5);
    println!("The sum is: {}", sum);

    let difference = calculator::multiply(10, 5);
    println!("The difference is: {}", difference);

    // 
    greetings::formal::hello();
    greetings::welcome();
    greetings::hello();
    greetings::goodbye();
}

fn print_color(c: colors::Color) {
    match c {
       colors::Color::Blue => println!("This is Blue"),
       colors::Color::Red => println!("This is Red"), 
       colors::Color::Green => println!("This is Green"), 
       colors::Color::RGB(r, g, b) => println!("This is RGB({},{},{})",r,g,b), 

    }
}

