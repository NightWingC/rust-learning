fn main(){
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectagule is {} square pixels", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32{
    width * height
}

//refactorin with tuples
fn main(){
    let rect1 = (30,50);
    println!("The area of the rectagule is {} square pixels", area(rect1));
}

fn area(dimensions: (u32,u32)) -> u32{
    dimensions.0 * dimensions.1
}

// Refactoring with structs 
struct Rectangule {
    width: u32,
    height: u32
}
fn main(){
    let rect = Rectangule { width: 30, height: 50 };
    println!("The area of the rectagule is {} square pixels", area(&rect));
}

fn area(rectangule: &Rectangule) -> u32{
    rectangule.width * rectangule.height
}


#[derive(Debug)]
struct Rectangule {
    width: u32,
    height: u32
}

fn main(){
    let rect = Rectangule { width: 30, height: 50 };
    println!("The area of the rectagule is {} square pixels", area(&rect));

    let rect1 = Rectangule { width: 30, height: 50 };

    println!("rect1 is : {:?}", rect1);
}

fn area(rectangule: &Rectangule) -> u32{
    rectangule.width * rectangule.height
}

// Method sytax
#[derive(Debug)]
struct Rectangule {
    width: u32,
    height: u32
}

impl Rectangule {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}
fn main(){
    let rect = Rectangule { width: 30, height: 50 };
    println!("The area of the rectagule is {} square pixels", rect.area());

    if rect.width(){
        println!("The rectangule has nonzero width; it is {} ", rect.width);
    }

    let rect1 = Rectangule { width: 30, height: 50 };

    println!("rect1 is : {:?}", rect1);
}

// Methods with more parameters
#[derive(Debug)]
struct Rectangule {
    width: u32,
    height: u32
}

impl Rectangule {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangule) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}
fn main(){
    let rect1 = Rectangule { width: 30, height: 50 };
    let rect2 = Rectangule { width: 10, height: 40 };
    let rect3 = Rectangule { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangule::square(20);

}

// Associated Functions
