use rocket::{get, routes};

#[get("/")]
fn greeting() -> &'static str {
    "Hi from /greeting"
}

#[get("/<name>")]
fn greeting_params(name: String) -> String {
    format!("Hi from /greeting {}", name)
}
#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    rocket::build()
    .mount("/", routes![
        greeting, 
        greeting_params,
    ])
    .launch()
    .await?;

    Ok(())
}
