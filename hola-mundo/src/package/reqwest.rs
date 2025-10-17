#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // HAcer peticion GET a un URL
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?
        .text()
        .await?;
    println!("Respuesta: {}", response);
    Ok(())
}