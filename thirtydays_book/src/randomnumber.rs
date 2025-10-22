// First example 
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..=100);
    println!("Generated random number: {}", n);
}
