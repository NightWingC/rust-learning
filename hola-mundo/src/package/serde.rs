use serde::{ Deserialize, Serialize };
use serde_json;

#[derive(Deserialize, Serialize, Debug)]

struct Persona {
    nombre: String,
    edad: u8,
}
fn main() {
   let p = Persona { nombre: "Alice".to_string(), edad: 30};

    //    Serializar al JSON
   let json_string = serde_json::to_string(&p).unwrap();
   println!("JSON: {}", json_string);

    //    Deserializar de json a struct
    let p2 : Persona = serde_json::from_str(&json_string).unwrap();
    println!("Struct: {:?}", p2);
}