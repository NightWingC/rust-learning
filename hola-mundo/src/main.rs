fn doble(x: i32) -> i32 { x * 2}
fn mas_cinco(x: i32) -> i32 { x + 5}

fn comp<F,G>(f: F, g: G) -> impl Fn(i32) -> i32 
    where 
        F: Fn(i32) -> i32,
        G: Fn(i32) -> i32,
        {
            move| x | g(f(x)) 
        }
fn main(){
    let doble_mas_cinco = comp(doble, mas_cinco);
    println!("Resultado: {}", doble_mas_cinco(3))
}
