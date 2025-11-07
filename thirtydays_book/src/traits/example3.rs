use std::fmt::Display;

struct MuDisplayable {
    value: i32,
}

impl Display for MuDisplayable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyDisplayable({})", self.value)
    }
}

fn print_debug_only<T: std::fmt::Debug>(item: T){
    println!("{:?}", item);
}

fn main() {
    let my_item = MuDisplayable { value: 42 };
    
}