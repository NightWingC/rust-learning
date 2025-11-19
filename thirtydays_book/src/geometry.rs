pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn get_coords(&self) -> (i32,i32) {
        (self.x, self.y)
    }
}