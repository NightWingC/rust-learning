use tokio::time::{ sleep, Duration };

#[tokio::main]
async fn main(){
    println!("Comienzo async");
    sleep(Duration::from_secs(1)).await;
    println!("Termino");
}