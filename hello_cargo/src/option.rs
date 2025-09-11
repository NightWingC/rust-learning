
// Option
// enum Option <T>{
//     Some(T), 
//     None,
// }

// Null
fn main(){
    let mut nombre : Option<String> = None;
    nombre = Some("Julio".to_string());
    
    match nombre {
        None => println!("No hay valor"),
        Some(nombre) => println!("{}", nombre),
    }
    
    let new = User {
        name: "Julio".to_string(),
        age: Some(12),
    };

    let age = new.get_age();
    match age {
        Some(age) => println!("Tengo {}", age),
        _ => (),
    }
    
}

struct User {
    name: String,
    age: Option<i32>,
}

impl User {
    fn get_age(&self) -> Option<i32> {
        self.age
    }
}