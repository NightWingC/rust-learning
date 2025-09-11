fn main(){
    // Generics
    let pointA = Point {
        x: 0.45,
        y: 12,
    };

    let pointB = Point {
        x: "12",
        y: 1,
    };
}

struct Point <T, V> {
    x: T,
    y: T,
}

fn calculate_thing( pointA: Point<T>, pointB: Point<V>){
    
}