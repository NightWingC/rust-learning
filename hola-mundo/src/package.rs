use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let numero_aleatorio: i32 = rng.gen_range(1..=100);

    println!("Numero aleatorio: {numero_aleatorio}");
}