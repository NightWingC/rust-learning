use miniserde::json;

#[derive(miniserde::Serialize, miniserde::Deserialize)]
struct Persona {
    nombre: String,
    edad: u8,
}
fn main() {
    let p = Persona { nombre: String::from("ALice"), edad: 30 };
    let json_str = json::to_string(&p);
    println!("Persona en JSON: {}", json_str);

    let p2 : Persona = json::from_str(&json_str).unwrap();
    println!("Nombre:{} Edad: {}", p2.nombre, p2.edad);
}